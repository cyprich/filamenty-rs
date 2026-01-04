import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import axios from "axios";
import config from "./config.json";
import EditFilament from "./EditFilament.jsx";
import DeleteFilament from "./DeleteFilament.jsx";

// TODO make user able to change color

function Filament() {
  const IP = config.ip;
  const { id } = useParams();

  const [filament, setfilament] = useState([]);
  const [responseCode, setResponseCode] = useState(0);

  const [showDelete, _setShowDelete] = useState(false);

  useEffect(() => {
    axios
      .get(`http://${IP}:5000/api/v2/filaments/${id}/`)
      .then((response) => {
        setfilament(response.data);
        setResponseCode(response.status);
      })
      .catch((error) => console.error(error));
  }, [IP, id]);

  function setShowDelete(state) {
    _setShowDelete(!showDelete);
    _setShowDelete(state);
  }

  return (
    <div className={"main filament flex flex-col gap-8 portrait:!gap-6"}>
      {responseCode === 200 ? (
        <>
          <div className={"flex items-center gap-8 portrait:justify-between"}>
            <img
              className={"h-24"}
              src={`http://${IP}:5000/api/v2/image/qr${id}.png`}
              alt=""
            />
            <div className={"portrait:text-right"}>
              <div className={"flex gap-3 portrait:justify-end"}>
                <h2 className={"!text-3xl font-bold portrait:!text-2xl"}>
                  {filament.name_vendor}
                </h2>
                <h2 className={"!text-3xl font-extralight portrait:hidden"}>
                  #{filament.id_filament}
                </h2>
              </div>
              <h2
                className={
                  "hidden !text-3xl font-extralight portrait:!text-xl portrait:block"
                }
              >
                #{filament.id_filament}
              </h2>
              <h2 className={"font-medium portrait:!text-xl"}>
                {filament.name_material}
              </h2>
            </div>
          </div>
          <div
            style={{
              background: `linear-gradient(to right, ${filament.color_hex}, ${filament.color_second_hex || filament.color_hex})`,
            }}
            className={"w-full h-1 rounded-full portrait:-mt-0"}
          />
          <div
            className={"flex gap-16 portrait:flex-col portrait:items-center"}
          >
            <img
              src={`http://${IP}:5000/api/v2/image/filament${filament.id_filament}.png`}
              className={
                "border custom-border w-128 h-128 aspect-square object-cover p-8 portrait:w-80 portrait:h-80 portrait:p-4"
              }
              alt=""
            />
            <div className={"w-full flex flex-col gap-4"}>
              <div className={"flex flex-col gap-3 portrait:items-center"}>
                <h2>Informácie</h2>
                <table className={"info-table w-max"}>
                  <tbody>
                    <tr>
                      <td>Minimálna teplota tlače</td>
                      <td>{filament.temp_min} °C</td>
                      <EditFilament
                        id={id}
                        fieldName={"temp_min"}
                        name={"Minimálna teplota tlače"}
                      />
                    </tr>
                    <tr>
                      <td>Maximálna teplota tlače</td>
                      <td>{filament.temp_max} °C</td>
                      <EditFilament
                        id={id}
                        fieldName={"temp_max"}
                        name={"Minimálna teplota tlače"}
                      />
                    </tr>
                    <tr>
                      <td>
                        {filament.temp_bed_max ? "Minimálna t" : "T"}eplota
                        podložky
                      </td>
                      <td>{filament.temp_bed_min} °C</td>
                      <EditFilament
                        id={id}
                        fieldName={"temp_bed_min"}
                        name={"Maximálna teplota tlače"}
                      />
                    </tr>
                    {filament.temp_bed_max && (
                      <tr>
                        <td>Maximálna teplota podložky</td>
                        <td>{filament.temp_bed_max} °C</td>
                        <EditFilament
                          id={id}
                          fieldName={"temp_bed_max"}
                          name={"Minimálna teplota podložky"}
                        />
                      </tr>
                    )}
                    <tr>
                      <td>
                        <b>Zostávajúca hmotnosť</b>
                      </td>
                      <td>
                        <b>{filament.netto_weight}{" g"}</b>
                      </td>
                      <EditFilament
                        id={id}
                        fieldName={"weight_full"}
                        additional_data={filament.spool_weight}
                        name={"Maximálna teplota podložky"}
                      />
                    </tr>
                    <tr>
                      <td>Hmotnosť so spoolom</td>
                      <td>{filament.netto_weight + filament.spool_weight} g</td>
                      <EditFilament
                        id={id}
                        fieldName={"weight"}
                        name={"Hmotnosť so spoolom"}
                      />
                    </tr>
                    <tr>
                      <td>Hmotnosť spoolu</td>
                      <td>{filament.spool_weight} g</td>
                      <EditFilament
                        id={id}
                        fieldName={"spool_weight"}
                        name={"Hmotnosť spoolu"}
                      />
                    </tr>
                    <tr>
                      <td>Pôvodná hmotnosť</td>
                      <td>{filament.original_weight} g</td>
                      <EditFilament
                        id={id}
                        fieldName={"original_weight"}
                        name={"Pôvodná hmotnosť"}
                      />
                    </tr>
                    <tr>
                      <td>Cena</td>
                      <td>{(filament.price || 0).toFixed(2)} €</td>
                      <EditFilament id={id} fieldName={"price"} name={"Cena"} />
                    </tr>
                    {filament.original_weight !== 1000 && (
                      <tr>
                        <td>Cena za 1kg</td>
                        <td>
                          {(
                            filament.price /
                            (filament.original_weight / 1000)
                          ).toFixed(2)}{" "}
                          €/kg
                        </td>
                        <EditFilament
                          id={id}
                          fieldName={"price_kg"}
                          additional_data={filament.original_weight}
                          name={"Cena za 1kg"}
                        />
                      </tr>
                    )}
                  </tbody>
                  <tfoot>
                    <tr className={"portrait:hidden"}>
                      <td
                        className={
                          "flex mt-4 justify-center items-center cursor-pointer border rounded-lg border-red-600 hover:bg-red-600/10 select-none"
                        }
                        onClick={() => {
                          setShowDelete(true);
                        }}
                      >
                        <img
                          className={"w-8 -ml-2"}
                          src="/src/images/delete_red.png"
                          alt=""
                        />
                        <p>Odstrániť</p>
                      </td>
                    </tr>
                  </tfoot>
                </table>
                <div
                  className={
                    "hidden portrait:flex justify-center items-center gap-2 w-full border border-red-600 py-4 rounded-xl hover:bg-red-600/10 select-none"
                  }
                  onClick={() => {
                    setShowDelete(true);
                  }}
                >
                  <img
                    className={"w-8"}
                    src="/src/images/delete_red.png"
                    alt=""
                  />
                  <p>Odstrániť</p>
                </div>
              </div>
              {showDelete && (
                <DeleteFilament
                  id={filament.id}
                  image_url={`http://${IP}:5000/api/v2/filament${filament.id_filament}.png`}
                  setShowDelete={setShowDelete}
                />
              )}
            </div>
          </div>
        </>
      ) : (
        <div className={"flex flex-col justify-center gap-3 pt-32 text-center"}>
          <h2 className={"!text-4xl font-bold"}>Filament sa nenašiel</h2>
          <h2 className={"!text-3xl pb-2 font-semibold"}>:(</h2>
          <p>Pravdepodobne ste zadali zlé číslo filamentu</p>
        </div>
      )}
    </div>
  );
}

export default Filament;
