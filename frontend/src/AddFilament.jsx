import { useState } from "react";
import config from "./config.json";
import axios from "axios";
import { useNavigate } from "react-router-dom";

function AddFilament() {
  const IP = config.ip;
  let navigate = useNavigate();

  const [vendor, setVendor] = useState("");
  const [material, setMaterial] = useState("");
  const [color, setColor] = useState("#333333");
  const [secondColor, setSecondColor] = useState("#666666");
  const [toggleSecondColor, setToggleSecondColor] = useState(false);
  const [tempMin, setTempMin] = useState(0);
  const [tempMax, setTempMax] = useState(0);
  const [tempBedMin, setTempBedMin] = useState(0);
  const [tempBedMax, setTempBedMax] = useState(0);
  const [weight, setWeight] = useState(0);
  const [weightSpool, setWeightSpool] = useState(0);
  const [weightOrig, setWeightOrig] = useState(0);
  const [price, setPrice] = useState(0);
  const [image, setImage] = useState(null);
  const [imagePreview, setImagePreview] = useState(null);

  // text and number dependencies - required fields
  const [depsAreOK, setDepsAreOK] = useState(true);
  const textDeps = [vendor, material];
  const numDeps = [
    tempMin,
    tempMax,
    tempBedMin,
    tempBedMax,
    weight,
    weightSpool,
    weightOrig,
    price,
  ];

  function handleSubmit() {
    if (checkDeps()) {
      // https://uploadcare.com/blog/how-to-upload-file-in-react/
      const formData = new FormData();

      formData.append("vendor", vendor);
      formData.append("material", material);
      formData.append("color_hex", color);
      toggleSecondColor && formData.append("color_second_hex", secondColor);
      formData.append("temp_min", tempMin);
      formData.append("temp_max", tempMax);
      formData.append("temp_bed_min", tempBedMin);
      formData.append("temp_bed_max", tempBedMax);
      formData.append("weight", weight);
      formData.append("spool_weight", weightSpool);
      formData.append("original_weight", weightOrig);
      formData.append("original_weight", weightOrig);
      formData.append("original_weight", weightOrig);
      formData.append("original_weight", weightOrig);
      formData.append("original_weight", weightOrig);
      formData.append("price", price);
      formData.append("image", image);

      axios
        .post(`http://${IP}:5000/api/v2/filaments/`, formData, {
          headers: { "Content-Type": "multipart/form-data" },
        })
        .then((response) => {
          navigate(`/filament/${response.data.id_filament}`);
        })
        .catch((error) => console.error(error));
    }
  }

  function handleImageChange(event) {
    const file = event.target.files[0];
    setImage(file);
    setImagePreview(URL.createObjectURL(file));
  }

  function checkDeps() {
    const textDepsAreOK = textDeps.every((i) => i !== "");
    const numDepsAreOK = numDeps.every((i) => i != 0);
    const imageIsOK = image !== null;

    setDepsAreOK(textDepsAreOK && numDepsAreOK && imageIsOK);
    return textDepsAreOK && numDepsAreOK && imageIsOK;
  }

  return (
    <div
      className={
        "main add-filament flex gap-16 portrait:flex-col portrait:justify-center portrait:items-center portrait:text-center portrait:gap-8"
      }
    >
      <h1 className={"hidden portrait:block"}>Nový filament</h1>
      <div
        className={
          "custom-border border w-[40rem] h-[40rem] portrait:w-full portrait:h-auto portrait:!aspect-square"
        }
      >
        <div
          className={
            "relative flex justify-center items-center p-8 w-full h-full portrait:p-6"
          }
        >
          {imagePreview !== null ? (
            <>
              <img
                className={
                  "absolute object-contain w-full h-full p-8 drop-shadow-xl portrait:p-4"
                }
                src={imagePreview}
                alt=""
              />
              <img
                className={
                  "clickable absolute bottom-0 right-0 m-4 h-14 w-14 p-3 border drop-shadow-lg !rounded-full bg-zinc-100/95 portrait:w-12 portrait:h-12"
                }
                src={"src/images/delete.png"}
                alt=""
                onClick={() => {
                  setImage(null);
                  setImagePreview(null);
                }}
              />
            </>
          ) : (
            <label
              htmlFor="fileInput"
              className={
                "w-full h-full flex justify-center items-center cursor-pointer italic font-light bg-zinc-200 shadow-xl"
              }
            >
              Kliknutím zvoľte obrázok
            </label>
          )}

          <input
            type="file"
            id={"fileInput"}
            className={"hidden"}
            accept={"image/jpeg, image/png"}
            onChange={(e) => handleImageChange(e)}
          />
        </div>
      </div>
      <div className={"flex flex-col gap-8"}>
        <h1 className={"portrait:hidden"}>Nový filament</h1>
        <table>
          <tbody>
            <tr>
              <td>Výrobca</td>
              <td>
                <input
                  type="text"
                  onChange={(e) => setVendor(e.target.value)}
                  value={name_vendor}
                />
              </td>
            </tr>
            <tr>
              <td>Materiál</td>
              <td>
                <input
                  type="text"
                  onChange={(e) => setMaterial(e.target.value)}
                  value={name_material}
                />
              </td>
            </tr>
            <tr>
              <td>Farba</td>
              <td>
                <input
                  type="color"
                  onChange={(e) => setColor(e.target.value)}
                  value={color}
                />
                <button
                  className={"clickable-small font-bold"}
                  onClick={() => setToggleSecondColor(!toggleSecondColor)}
                >
                  {toggleSecondColor ? "-" : "+"}
                </button>
              </td>
            </tr>
            {toggleSecondColor && (
              <tr>
                <td>Sekundárna farba</td>
                <td>
                  <input
                    type="color"
                    onChange={(e) => setSecondColor(e.target.value)}
                    value={secondColor}
                  />
                </td>
              </tr>
            )}
            <tr>
              <td>Teplota tlače</td>
              <td>
                <input
                  type="number"
                  onChange={(e) => setTempMin(e.target.value)}
                  value={tempMin || ""}
                />
                <p>-</p>
                <input
                  type="number"
                  onChange={(e) => setTempMax(e.target.value)}
                  value={tempMax || ""}
                />
              </td>
            </tr>
            <tr>
              <td>Teplota podložky</td>
              <td>
                <input
                  type="number"
                  onChange={(e) => setTempBedMin(e.target.value)}
                  value={tempBedMin || ""}
                />
                <p>-</p>
                <input
                  type="number"
                  onChange={(e) => setTempBedMax(e.target.value)}
                  value={tempBedMax || ""}
                />
              </td>
            </tr>
            <tr>
              <td>Hmotnosť so spoolom</td>
              <td>
                <input
                  type="number"
                  onChange={(e) => setWeight(e.target.value)}
                  value={weight || ""}
                />
              </td>
            </tr>
            <tr>
              <td>Hmotnosť spoolu</td>
              <td>
                <input
                  type="number"
                  onChange={(e) => setWeightSpool(e.target.value)}
                  value={weightSpool || ""}
                />
              </td>
            </tr>
            <tr>
              <td>Pôvodná hmotnosť</td>
              <td>
                <input
                  type="number"
                  onChange={(e) => setWeightOrig(e.target.value)}
                  value={weightOrig || ""}
                />
              </td>
            </tr>
            <tr>
              <td>Cena</td>
              <td>
                <input
                  type="number"
                  onChange={(e) => setPrice(e.target.value)}
                  value={price || ""}
                />
              </td>
            </tr>
          </tbody>
        </table>
        <div className={"flex items-center gap-8 portrait:justify-center"}>
          <button
            className={"clickable-small w-max !px-16 !py-4"}
            onClick={() => handleSubmit()}
          >
            Potvrdiť
          </button>
          {depsAreOK || (
            <p className={"text-red-600 font-medium"}>
              Prosím vyplňte všetky polia
            </p>
          )}
        </div>
      </div>
    </div>
  );
}

export default AddFilament;
