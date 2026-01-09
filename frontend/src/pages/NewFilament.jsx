import {useState} from "react";
import {useNavigate} from "react-router-dom";

import axios from "axios";

import DropdownProduct from "../components/DropdownProduct.jsx";
import EditableImage from "../components/EditableImage.jsx";
import PageHeading from "../components/PageHeading.jsx";

import {BASE_URL} from "../config.js";

function NewFilament() {
    let navigate = useNavigate();

    const [idProduct, setIdProduct] = useState(0)
    const [colorName, setColorName] = useState("")
    const [colorHex, setColorHex] = useState("#4169e1");
    const [weight, setWeight] = useState(0);
    const [weightSpool, setWeightSpool] = useState(0);
    const [weightOrig, setWeightOrig] = useState(0);
    const [price, setPrice] = useState(0);
    const [imagePath, setImagePath] = useState("");

    // text and number dependencies - required fields
    const [depsAreOK, setDepsAreOK] = useState(true);
    const textDeps = [colorName, colorHex];
    const numDeps = [
        idProduct,
        weight,
        weightSpool,
        weightOrig,
        price,
    ];

    function handleSubmit() {
        if (checkDeps()) {
            axios
                .post(`${BASE_URL}/filaments/`, {
                    "id_product": Number(idProduct),
                    "price": parseFloat(price),
                    "color_name": colorName,
                    "color_hex": colorHex,
                    "original_weight": Number(weightOrig),
                    "spool_weight": Number(weightSpool),
                    "netto_weight": Number(weight - weightSpool),
                    "image_path": imagePath
                })
                .then((response) => {
                    navigate(`/filament/${response.data.id_filament}`);
                })
                .catch((error) => console.error(error));
        }
    }

    function checkDeps() {
        const textDepsAreOK = textDeps.every((i) => i !== "");
        const numDepsAreOK = numDeps.every((i) => i != 0);

        setDepsAreOK(textDepsAreOK && numDepsAreOK);
        return textDepsAreOK && numDepsAreOK;
    }

    return (
        <div className={"main main-spacing add-filament flex flex-col"}>
            <PageHeading title={"Nový filament"}/>
            <div className={"flex gap-8"}>
                <EditableImage defaultSrc={`${BASE_URL}/images/missing_filament.png`} setImagePath={setImagePath}/>
                <div className={"flex flex-col justify-between"}>
                    <table>
                        <tbody>
                        <tr>
                            <td>Produkt</td>
                            <td>
                                <div className={"-ml-4"}>
                                    <DropdownProduct value={idProduct} setIdProduct={setIdProduct}/>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>Farba</td>
                            <td>
                                <input
                                    type="text"
                                    onChange={(e) => setColorName(e.target.value)}
                                />
                                <input
                                    type="color"
                                    onChange={(e) => setColorHex(e.target.value)}
                                    value={colorHex}
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
                        >Potvrdiť
                        </button>
                        {depsAreOK || (
                            <p className={"text-red-600 font-medium"}>
                                Prosím vyplňte všetky polia
                            </p>
                        )}
                    </div>
                </div>
            </div>
        </div>
    );
}

export default NewFilament;
