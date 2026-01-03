import { useEffect, useState } from "react";

import config from "./config.json";
import axios from "axios";
import { useNavigate } from "react-router-dom";

function Labels() {
  const IP = config.ip;
  const [filaments, setfilaments] = useState([]);

  let navigate = useNavigate();

  useEffect(() => {
    axios
      .get(`http://${IP}:5000/api/v2/filaments_full/`)
      .then((response) => {
        setfilaments(response.data);
      })
      .catch((error) => console.error(error));
  }, [IP]);

  return (

    <div
      className={
        "main labels flex flex-col items-center gap-16 portrait:gap-8 portrait:!px-8"
      }
    >
      <h1 className={"no-print"}>Štítky</h1>
      <div
        className={
          "grid grid-cols-4 gap-4 print:grid-cols-2 portrait:grid-cols-1"
        }
      >
        {filaments.map((item, key) => {
          return (
            <div
              className={
                "no-break flex justify-between items-center gap-8 w-max p-8 border portrait:gap-5 portrait:p-4"
              }
              key={key}
            >
              <div className={"flex flex-col gap-3 portrait:gap-1.5"}>
                <div className={"flex gap-3 items-center"}>
                  <div
                    className={
                      "w-12 h-12 rounded-full portrait:w-8 portrait:h-8"
                    }
                    style={{
                      backgroundImage: `linear-gradient(135deg, ${item.color_hex}, ${item.color_second_hex || item.color_hex})`,
                    }}
                  />
                  <div>
                    <p
                      className={"text-xl flex gap-4 justify-between leading-5"}
                    >
                      <span className={"font-extrabold portrait:font-bold"}>
                        {item.name_material}
                      </span>
                      <span className={"font-extralight"}>#{item.id_filament}</span>
                    </p>
                    <p className={"portrait:!text-sm"}>{item.name_vendor}</p>
                  </div>
                </div>
                <table className={"info-table"}>
                  <tbody>
                    <tr>
                      <td>Temp:</td>
                      <td>
                        {item.temp_min} - {item.temp_max} °C
                      </td>
                    </tr>
                    <tr>
                      <td>Bed:</td>
                      <td>
                        {item.temp_bed_min}{" "}
                        {item.temp_bed_max && `- ${item.temp_bed_max}`}°C
                      </td>
                    </tr>
                    <tr>
                      <td>Price:</td>
                      <td>{(item.price || 0).toFixed(2)} €</td>
                    </tr>
                  </tbody>
                </table>
              </div>
              <img
                className={
                  "clickable-small w-32 h-32 portrait:w-24 portrait:h-24"
                }
                src={`http://${IP}:5000/api/v2/image/qr${item.id_filament}.png/`}
                onClick={() => navigate(`/filament/${item.id_filament}`)}
                alt=""
              />
            </div>
          );
        })}
      </div>
      <p
        className={
          "no-print clickable-small custom-border border -my-6 cursor-pointer px-16 py-5 shadow-lg bg-zinc-50 text-center portrait:-my-0 portrait:w-full"
        }
        onClick={() => window.print()}
      >
        Tlačiť
      </p>
      <p className={"no-print text-[0.9rem] -mt-4 text-center"}>
        Pre optimálny výsledok je potrebné{" "}
        <span className={"font-semibold"}>zapnúť tlačenie pozadia</span> (Print
        Backgrounds)
      </p>
    </div>
  );
}

export default Labels;
