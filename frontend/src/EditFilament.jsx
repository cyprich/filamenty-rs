import config from "./config.json";
import { useState } from "react";
import axios from "axios";

function EditFilament({ id, fieldName, name, additional_data }) {
  const IP = config.ip;
  const [toggleEdit, setToggleEdit] = useState(false);
  const [value, setValue] = useState(0);

  function generate_data() {
    if (fieldName === "weight_full") {
      return {
        key: "weight",
        value: value + additional_data,
      };
    } else if (fieldName === "price_kg") {
      return {
        key: "price",
        value: value * (additional_data / 1000),
      };
    } else {
      return {
        key: fieldName,
        value: value,
      };
    }
  }

  function handleChange() {
    axios
      .put(`http://${IP}:5000/api/v2/filaments/${id}/`, generate_data())
      .then((response) => {
        window.location.reload();
      })
      .catch((error) => {
        console.error(error);
      });
  }

  return (
    <td className={"flex items-center h-full"}>
      {toggleEdit ? (
        <>
          <div className={"flex gap-4 portrait:hidden"}>
            <input
              className={"border rounded-xl p-1"}
              type="number"
              onChange={(e) => setValue(Number(e.target.value))}
            />
            <button
              className={"clickable-small border"}
              onClick={() => handleChange()}
            >
              Potvrdiť
            </button>
            <button
              className={"clickable-small border"}
              onClick={() => setToggleEdit(false)}
            >
              Zrušiť
            </button>
          </div>
          <div
            className={
              "hidden portrait:flex portrait:justify-center portrait:items-center w-screen h-screen fixed top-0 left-0 bg-black/50 backdrop-blur-xs "
            }
          >
            <div
              className={
                "custom-border flex flex-col gap-4 w-[80%] h-max p-8 bg-zinc-100 outline outline-zinc-100 outline-offset-16 drop-shadow-lg"
              }
            >
              <div className={"flex flex-col justify-center items-center "}>
                <h2 className={"!text-xl font-semibold"}>
                  Zadajte novú hodnotu
                </h2>
                <p className={"font-medium"}>{name}</p>
                <p className={"font-light"}>Filament #{id}</p>
              </div>
              <input
                className={"border rounded-xl p-1"}
                type="number"
                onChange={(e) => setValue(Number(e.target.value))}
              />
              <div className={"flex justify-center gap-4"}>
                <button
                  className={"clickable-small"}
                  onClick={() => handleChange()}
                >
                  Potvrdiť
                </button>
                <button
                  className={"clickable-small"}
                  onClick={() => setToggleEdit(false)}
                >
                  Zrušiť
                </button>
              </div>
            </div>
          </div>
        </>
      ) : (
        <img
          className={"clickable w-6 h-6"}
          src={"/src/images/edit.png"}
          alt=""
          onClick={() => setToggleEdit(!toggleEdit)}
        />
      )}
    </td>
  );
}

export default EditFilament;
