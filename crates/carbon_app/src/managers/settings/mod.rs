use std::path::PathBuf;

use anyhow::{anyhow, Context};
use chrono::Utc;
use itertools::Itertools;
use reqwest_middleware::ClientWithMiddleware;

use crate::{
    api::{keys::settings::*, settings::FESettingsUpdate},
    db::app_configuration::{self},
    domain::{self as domain, modplatforms::ModChannelWithUsage, runtime_path},
};

use super::ManagerRef;

use self::terms_and_privacy::TermsAndPrivacy;

pub mod terms_and_privacy;

pub(crate) struct SettingsManager {
    pub runtime_path: runtime_path::RuntimePath,
    pub terms_and_privacy: TermsAndPrivacy,
}

impl SettingsManager {
    pub fn new(runtime_path: PathBuf, http_client: ClientWithMiddleware) -> Self {
        Self {
            runtime_path: runtime_path::RuntimePath::new(runtime_path),
            terms_and_privacy: TermsAndPrivacy::new(http_client),
        }
    }
}

impl ManagerRef<'_, SettingsManager> {
    pub async fn get_settings(self) -> anyhow::Result<crate::db::app_configuration::Data> {
        self.app
            .prisma_client
            .app_configuration()
            .find_unique(app_configuration::id::equals(0))
            .exec()
            .await?
            .ok_or(anyhow!("Can't find this key"))
    }

    #[tracing::instrument(skip(self))]
    pub async fn set_settings(self, incoming_settings: FESettingsUpdate) -> anyhow::Result<()> {
        let db = &self.app.prisma_client;

        let crate::db::app_configuration::Data {
            secret,
            random_user_uuid,
            ..
        } = self.get_settings().await?;

        let random_user_uuid = uuid::Uuid::parse_str(&random_user_uuid)?;

        let mut queries = vec![];

        if let Some(theme) = incoming_settings.theme {
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::theme::set(theme.inner())],
            ));
        }

        if let Some(language) = incoming_settings.language {
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::language::set(language.inner())],
            ));
        }

        if let Some(reduced_motion) = incoming_settings.reduced_motion {
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::reduced_motion::set(
                    reduced_motion.inner(),
                )],
            ));
        }

        if let Some(discord_integration) = incoming_settings.discord_integration {
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::discord_integration::set(
                    discord_integration.inner(),
                )],
            ));
        }

        if let Some(release_channel) = incoming_settings.release_channel {
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::release_channel::set(
                    release_channel.inner().into(),
                )],
            ));
        }

        if let Some(launcher_action_on_game_launch) =
            incoming_settings.launcher_action_on_game_launch
        {
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::launcher_action_on_game_launch::set(
                    launcher_action_on_game_launch.inner().into(),
                )],
            ));
        }

        if let Some(show_app_close_warning) = incoming_settings.show_app_close_warning.clone() {
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::show_app_close_warning::set(
                    show_app_close_warning.inner(),
                )],
            ));
        }

        if let Some(concurrent_downloads) = incoming_settings.concurrent_downloads {
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::concurrent_downloads::set(
                    concurrent_downloads.inner(),
                )],
            ));
        }

        if let Some(download_dependencies) = incoming_settings.download_dependencies {
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::download_dependencies::set(
                    download_dependencies.inner(),
                )],
            ));
        }

        if let Some(show_news) = incoming_settings.show_news {
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::show_news::set(show_news.inner())],
            ));
        }

        if let Some(sort_by) = incoming_settings.instances_sort_by {
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::instances_sort_by::set(
                    sort_by.inner().into(),
                )],
            ));
        }

        if let Some(instances_sort_by_asc) = incoming_settings.instances_sort_by_asc {
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::instances_sort_by_asc::set(
                    instances_sort_by_asc.inner(),
                )],
            ));
        }

        if let Some(instances_group_by) = incoming_settings.instances_group_by {
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::instances_group_by::set(
                    instances_group_by.inner().into(),
                )],
            ));
        }

        if let Some(instances_group_by_asc) = incoming_settings.instances_group_by_asc {
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::instances_group_by_asc::set(
                    instances_group_by_asc.inner(),
                )],
            ));
        }

        if let Some(instances_tile_size) = incoming_settings.instances_tile_size {
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::instances_tile_size::set(
                    instances_tile_size.inner().into(),
                )],
            ));
        }

        if let Some(deletion_through_recycle_bin) = incoming_settings.deletion_through_recycle_bin {
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::deletion_through_recycle_bin::set(
                    deletion_through_recycle_bin.inner(),
                )],
            ));
        }

        if let Some(xmx) = incoming_settings.xmx {
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::xmx::set(xmx.inner())],
            ));
        }

        if let Some(xms) = incoming_settings.xms {
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::xms::set(xms.inner())],
            ));
        }

        if let Some(is_first_launch) = incoming_settings.is_first_launch {
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::is_first_launch::set(
                    is_first_launch.inner(),
                )],
            ));
        }

        if let Some(game_resolution) = incoming_settings.game_resolution {
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::game_resolution::set(
                    game_resolution.inner().map(Into::into),
                )],
            ));
        }

        if let Some(java_custom_args) = incoming_settings.java_custom_args {
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::java_custom_args::set(
                    java_custom_args.inner(),
                )],
            ));
        }

        if let Some(pre_launch_hook) = incoming_settings.pre_launch_hook {
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::pre_launch_hook::set(
                    pre_launch_hook.inner(),
                )],
            ));
        }

        if let Some(post_exit_hook) = incoming_settings.post_exit_hook {
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::post_exit_hook::set(
                    post_exit_hook.inner(),
                )],
            ));
        }

        if let Some(wrapper_command) = incoming_settings.wrapper_command {
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::wrapper_command::set(
                    wrapper_command.inner(),
                )],
            ));
        }

        if let Some(auto_manage_java_system_profiles) =
            incoming_settings.auto_manage_java_system_profiles.as_ref()
        {
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::auto_manage_java_system_profiles::set(
                    auto_manage_java_system_profiles.clone().inner(),
                )],
            ));
        }

        if let Some(mod_sources) = incoming_settings.mod_sources {
            let mod_sources = mod_sources.inner();

            let platform_blacklist = mod_sources
                .platform_blacklist
                .into_iter()
                .map(domain::modplatforms::ModPlatform::from)
                .map(|p| domain::modplatforms::ModPlatform::as_str(&p))
                .join(",");

            let channels = mod_sources
                .channels
                .into_iter()
                .map(Into::into)
                .collect::<Vec<_>>();
            ModChannelWithUsage::validate_list(&channels)?;

            let channels_str = ModChannelWithUsage::slice_to_str(&channels);

            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![
                    app_configuration::mod_platform_blacklist::set(platform_blacklist),
                    app_configuration::mod_channels::set(channels_str),
                ],
            ));
        }

        if let Some(terms_and_privacy_accepted) = incoming_settings.terms_and_privacy_accepted {
            let terms_and_privacy_accepted = terms_and_privacy_accepted.inner();
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![app_configuration::terms_and_privacy_accepted::set(
                    terms_and_privacy_accepted,
                )],
            ));

            let checksum = self
                .terms_and_privacy
                .record_consent(
                    terms_and_privacy::ConsentType::TermsAndPrivacy,
                    terms_and_privacy_accepted,
                    &random_user_uuid,
                    &secret,
                )
                .await
                .with_context(|| {
                    format!(
                        "Failed to record terms and privacy consent: {}",
                        terms_and_privacy_accepted
                    )
                })?;

            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![
                    app_configuration::terms_and_privacy_accepted::set(true),
                    app_configuration::terms_and_privacy_accepted_checksum::set(Some(
                        checksum.to_string(),
                    )),
                ],
            ));
        }

        if let Some(metrics_enabled) = incoming_settings.metrics_enabled {
            let metrics_enabled = metrics_enabled.inner();
            queries.push(self.app.prisma_client.app_configuration().update(
                app_configuration::id::equals(0),
                vec![
                    app_configuration::metrics_enabled::set(metrics_enabled),
                    app_configuration::metrics_enabled_last_update::set(Some(Utc::now().into())),
                ],
            ));

            self.terms_and_privacy
                .record_consent(
                    terms_and_privacy::ConsentType::Metrics,
                    metrics_enabled,
                    &random_user_uuid,
                    &secret,
                )
                .await
                .with_context(|| {
                    format!("Failed to record metrics consent: {}", metrics_enabled)
                })?;
        }

        if !queries.is_empty() {
            db._batch(queries).await?;
            self.app.invalidate(GET_SETTINGS, None);

            if let Some(show_app_close_warning) = incoming_settings.show_app_close_warning {
                println!(
                    "_SHOW_APP_CLOSE_WARNING_:{}",
                    show_app_close_warning.inner()
                );
            }
        }

        if let Some(auto_manage_java_system_profiles) =
            incoming_settings.auto_manage_java_system_profiles
        {
            if auto_manage_java_system_profiles.inner() {
                super::java::scan_and_sync::sync_system_java_profiles(db).await?;
            }
        }

        Ok(())
    }

    pub async fn set(self, value: app_configuration::SetParam) -> anyhow::Result<()> {
        self.app
            .prisma_client
            .app_configuration()
            .update(app_configuration::id::equals(0), vec![value])
            .exec()
            .await?;

        Ok(())
    }
}
