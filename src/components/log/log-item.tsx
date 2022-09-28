import { ApiType } from "../../services/types";

interface Props {
  value: ApiType.Registry;
}

const Registry = (props: Props) => {
  const { value } = props;

  return (
    <>
      <td className="path">{value.path.replace("HKEY_CLASSES_ROOT/", "") || "-"}</td>
      <td className="description">{value.description || "-"}</td>
      <td className="command">{value.command || "-"}</td>
      <td className="command">{value.modified_at || "-"}</td>
    </>
  );
};

export default Registry;
