import {useEffect, useState} from "react";
import axios from "axios";

import {BASE_URL} from "../config.js";

function DropdownMaterial({value, idProduct, setIdMaterial}) {
    const [materials, setMaterials] = useState([])

    useEffect(() => {
        axios
            .get(`${BASE_URL}/materials`)
            .then(response => setMaterials(response.data))
    }, []);

    function handleChange(selectedItem) {
        if (idProduct != undefined) {
            axios
                .patch(`${BASE_URL}/products/${idProduct}`, {
                    "key": "id_material",
                    "value": selectedItem
                })
                .then(() => window.location.reload())
                .catch(error => console.error(error))
        }

        if (setIdMaterial != undefined) {
            setIdMaterial(selectedItem)
        }
    }

    return (
        <select
            name="materialdropdown"
            value={value}
            onChange={(e) => handleChange(e.target.value)}>
            {
                setIdMaterial && <option value={0} disabled>Vyberte...</option>
            }
            {materials.map((item, key) => { return (
                <option
                    key={key}
                    value={item.id_material}>
                    {item.name_material}
                </option>
            )})}
        </select>
    )
}

export default DropdownMaterial


