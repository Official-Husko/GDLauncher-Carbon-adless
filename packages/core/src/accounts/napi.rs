use napi::bindgen_prelude::*;
use napi_derive::napi;

use super::{ms_auth::DeviceCode, Account, Accounts, ACCOUNTS};

#[napi(object, js_name = "account")]
struct NAPIAccount {
    pub id: String,
    pub name: String,
}

#[napi(object, js_name = "accounts")]
struct NAPIAccounts {
    pub accounts: Vec<NAPIAccount>,
    pub selected_account: Option<NAPIAccount>,
}

impl From<Accounts> for NAPIAccounts {
    fn from(accounts: Accounts) -> Self {
        let accounts_new: Vec<NAPIAccount> = accounts
            .inner
            .into_iter()
            .map(|account| NAPIAccount {
                id: account.mc_profile.id.clone(),
                name: account.mc_profile.name.clone(),
            })
            .collect();

        // Get index of selected account
        let selected_account = accounts.selected_account.map(|account| NAPIAccount {
            id: account.mc_profile.id.clone(),
            name: account.mc_profile.name.clone(),
        });

        NAPIAccounts {
            accounts: accounts_new,
            selected_account,
        }
    }
}

#[napi(object)]
struct DeviceCodeObject {
    pub user_code: String,
    pub link: String,
    pub expires_at: i64,
}

#[napi(ts_return_type = "Promise<String>")]
pub fn auth(
    env: Env,
    #[napi(ts_arg_type = "(deviceData: DeviceCodeObject) => void")] reporter: JsFunction,
) -> napi::Result<napi::JsObject> {
    // creating a promise which we can later resolve from another thread
    let (deferred, promise) = env.create_deferred()?;

    // wrapping the reporter callback with a threadsafe function
    let tsfn: napi::threadsafe_function::ThreadsafeFunction<
        DeviceCodeObject, // changethis value to whatever you need, it is the parameter type of the function
        napi::threadsafe_function::ErrorStrategy::Fatal,
    > = reporter.create_threadsafe_function(
        0,
        |ctx: napi::threadsafe_function::ThreadSafeCallContext<DeviceCodeObject>| {
            // this callback transforms the input we get in the tsfn.call() into napi values
            Ok(vec![ctx.value])
        },
    )?;

    env.execute_tokio_future(
        async move {
            let client = reqwest::Client::new();
            let device_code = DeviceCode::new(&client).await.unwrap();

            // this is how you can call the threadsafe function form another thread
            tsfn.call(
                DeviceCodeObject {
                    user_code: device_code.clone().inner.unwrap().user_code.clone().clone(),
                    link: device_code.clone().inner.unwrap().verification_uri.clone(),
                    expires_at: device_code.clone().expires_at.clone().unwrap().timestamp_millis()
                },
                napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking,
            );

            println!("device_code: {:?}", device_code);
            let auth = device_code.poll_device_code_auth(&client).await.unwrap();

            let mc_auth = auth.finalize_auth(&client).await.unwrap();
            let mc_profile = mc_auth.get_mc_profile(&client).await.unwrap();

            let account = Account {
                ms_data: auth,
                mc_data: mc_auth,
                mc_profile,
            };

            println!("{:?}", account.ms_data.get_id_token_claims().await.unwrap());
            let accounts = &*ACCOUNTS.lock().await;

            accounts.clone().add_account(account).await;

            // here the promise which we returned gets resolved with a computed value
            deferred.resolve(|_| Ok("Some stuff"));

            Ok(())
        },
        // this resolver converts the output of our async block to a js value which gets passed to the .then() callback is JS
        |&mut env, _| env.get_undefined(),
    )
    .unwrap();

    // here we instantly return a promise object which later can be resolved
    Ok(promise)
}

#[napi]
pub async fn init_accounts() -> Result<NAPIAccounts> {
    let accounts = Accounts::new().await.unwrap();
    Ok(accounts.into())
}
