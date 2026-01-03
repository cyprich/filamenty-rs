import axios from "axios";
import { useEffect } from "react";

import config from "./config.json";
import { useNavigate } from "react-router-dom";

function DeleteFilament({ id, image_url, setShowDelete }) {
  const IP = config.ip;

  let navigate = useNavigate();

  useEffect(() => {
    document.body.style.overflow = "hidden";

    return () => {
      document.body.style.overflow = "auto";
    };
  }, []);

  function handleDelete() {
    axios
      .delete(`http://${IP}:5000/api/v2/filaments/${id}/`)
      .then((response) => {
        navigate("/");
        window.location.reload();
      })
      .catch((error) => console.error(error));
  }

  return (
    <div
      className={
        "fixed top-0 left-0 flex justify-center items-center w-screen h-screen bg-black/50 backdrop-blur-xs"
      }
    >
      <div
        className={
          "custom-border flex flex-col justify-center items-center gap-4 p-8 bg-zinc-100 outline outline-zinc-100 outline-offset-16 portrait:w-[80%] portrait:p-4"
        }
      >
        <h2 className={"font-semibold"}>Odstrániť?</h2>
        <img src={image_url} alt="" className={"w-64 portrait:w-48"} />
        <div style={{ textAlign: "center" }}>
          <p>Naozaj si prajete odstrániť tento filament?</p>
          <p>Tento krok sa nedá vrátiť späť</p>
        </div>
        <div style={{ display: "flex", gap: "1em" }}>
          <button
            className={
              "clickable-small border-2 border-red-600/25 hover:border-red-600 portrait:border-red-600/80"
            }
            onClick={() => handleDelete()}
          >
            Odstrániť
          </button>
          <button
            className={"clickable-small"}
            onClick={() => setShowDelete(false)}
          >
            Zrušiť
          </button>
        </div>
      </div>
    </div>
  );
}

export default DeleteFilament;
