import { useTransContext } from "@gd/i18n";
import { Switch } from "@gd/ui";
import { setPayload, payload } from "..";

const FilesBundle = () => {
  const [t] = useTransContext();
  const handleSwitch = () => {
    setPayload({ ...payload, link_mods: !payload.link_mods });
  };
  return (
    <div class="w-full flex justify-between items-center pt-4">
      <span>{t("instance.bundle_files_text")}</span>
      <Switch onChange={handleSwitch} checked={payload.link_mods} />
    </div>
  );
};
export default FilesBundle;