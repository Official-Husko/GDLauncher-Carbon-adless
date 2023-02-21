import { Trans } from "@gd/i18n";
import { Show } from "solid-js";
import { format } from "date-fns";

type TypeProps = {
  title: string;
  mcversion: string;
  modloader: string;
  date: string;
  stable: string;
  isActive: boolean;
};

type Props = {
  version: TypeProps;
};

const getColor = (stable: string) => {
  switch (stable) {
    case "stable":
      return "text-green";
    case "beta":
      return "text-yellow";
    case "alpha":
      return "text-red";
    default:
      return "text-green";
  }
};

const Active = () => {
  return (
    <div class="flex items-center gap-2 cursor-pointer text-green">
      <Trans
        key="active_version"
        options={{
          defaultValue: "Active",
        }}
      />
      <div class="text-green text-2xl i-ri:check-fill" />
    </div>
  );
};

const Version = (props: Props) => {
  return (
    <div class="w-full h-14 flex items-center py-2 box-border">
      <div class="flex gap-4 justify-between items-center w-full">
        <div class="flex gap-4 justify-between items-center">
          <div class="flex items-center gap-2">
            <div class="flex flex-col">
              <p class="mt-0 mb-2">{props.version.title}</p>
              <div class="flex gap-2">
                <div class="m-0 text-sm flex items-center gap-2 text-shade-3">
                  {props.version.modloader} {props.version.mcversion}
                  <div class="h-2 w-px bg-shade-3" />
                  <p class="m-0 text-shade-3 text-md">
                    {format(new Date(props.version.date), "dd-MM-yyyy")}
                  </p>
                  <div class="h-2 w-px bg-shade-3" />
                  <span class={getColor(props.version.stable)}>
                    {props.version.stable}
                  </span>
                </div>
              </div>
            </div>
          </div>
        </div>
        <Show when={!props.version.isActive} fallback={<Active />}>
          <div class="group text-shade-3 transition ease-in-out flex items-center gap-2 cursor-pointer hover:text-shade-1">
            <Trans
              key="switch_version"
              options={{
                defaultValue: "Switch Version",
              }}
            />
            <div class="text-shade-3 text-2xl i-ri:download-2-line group-hover:text-shade-1" />
          </div>
        </Show>
      </div>
    </div>
  );
};

export default Version;