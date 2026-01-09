import PageHeading from "../components/PageHeading.jsx";
import {useNavigate} from "react-router-dom";
import {useState} from "react";
import DropdownVendor from "../components/DropdownVendor.jsx";
import DropdownMaterial from "../components/DropdownMaterial.jsx";
import axios from "axios";

import {BASE_URL} from "../config.js";

function NewProduct() {
    const navigate = useNavigate();

    const [idVendor, setIdVendor] = useState(0)
    const [idMaterial, setIdMaterial] = useState(0)
    const [nameProduct, setNameProduct] = useState("")
    const [diameter, setDiameter] = useState(1.75)
    const [tempMin, setTempMin] = useState(0)
    const [tempMax, setTempMax] = useState(0)
    const [tempBedMin, setTempBedMin] = useState(0)
    const [tempBedMax, setTempBedMax] = useState(0)

    // dependencies - required fields
    const [depsAreOK, setDepsAreOK] = useState(true);
    const textDeps = [nameProduct];
    const numDeps = [
        idVendor,
        idMaterial,
        diameter,
        tempMin,
        tempBedMin,
    ]

    function handleSubmit() {
        if (checkDeps()) {
            axios
                .post(`${BASE_URL}/products`, {
                    "id_vendor": Number(idVendor),
                    "id_material": Number(idMaterial),
                    "name_product": nameProduct,
                    "diameter": parseFloat(diameter),
                    "temp_min": Number(tempMin),
                    "temp_max": Number(tempMax),
                    "temp_bed_min": Number(tempBedMin),
                    "temp_bed_max": Number(tempBedMax),
                })
                .then(() => {
                    navigate(`/produkty`)
                })
                .catch(error => console.error(error))
        }
    }

    function checkDeps() {
        const textDepsAreOK = textDeps.every((i) => i !== "");
        const numDepsAreOK = numDeps.every((i) => i != 0);

        setDepsAreOK(textDepsAreOK && numDepsAreOK);
        return textDepsAreOK && numDepsAreOK;
    }

    return (
        <div className={"main main-spacing add-product"}>
            <PageHeading title={"Nový produkt"}/>
            <table>
                <tbody>
                <tr>
                    <td>Názov</td>
                    <td>
                        <input
                            type="text"
                            onChange={(e) => setNameProduct(e.target.value)}
                        />
                    </td>
                </tr>
                <tr>
                    <td>Výrobca</td>
                    <td>
                        <DropdownVendor value={idVendor} setIdVendor={setIdVendor}/>
                    </td>
                </tr>
                <tr>
                    <td>Materiál</td>
                    <td>
                        <DropdownMaterial value={idMaterial} setIdMaterial={setIdMaterial}/>
                    </td>
                </tr>
                <tr>
                    <td>Priemer</td>
                    <td>
                        <input
                            type="number"
                            onChange={(e) => setDiameter(e.target.value)}
                            value={diameter}
                        />
                    </td>
                </tr>
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
                </tbody>
            </table>
            <div className={"flex items-center gap-8"}>
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
    )
}

export default NewProduct