import {useEffect, useState} from "react";
import axios from "axios";

import {BASE_URL} from "../config.js";

function DropdownVendor({value, idProduct, setIdVendor}) {
    const [vendors, setVendors] = useState([])

    useEffect(() => {
        axios
            .get(`${BASE_URL}/vendors`)
            .then(response => {
                const data = response?.data;
                if (Array.isArray(data)) {
                    setVendors(data)
                } else {
                    setVendors([]);
                }
            })
    }, []);

    function handleChange(selectedItem) {
        if (idProduct != undefined) {
            axios
                .patch(`${BASE_URL}/products/${idProduct}`, {
                    "key": "id_vendor",
                    "value": selectedItem
                })
                .then(() => window.location.reload())
                .catch(error => console.error(error))
        }

        if (setIdVendor != undefined) {
            setIdVendor(selectedItem)
        }
    }

    return (
        <select
            name="vendordropdown"
            value={value}
            onChange={(e) => handleChange(e.target.value)}>
            {
                setIdVendor && <option value={0} disabled>Vyberte...</option>
            }
            {vendors.map((item, key) => { return (
                <option
                    key={key}
                    value={item.id_vendor}>
                    {item.name_vendor}
                </option>
            )})}
        </select>
    )
}

export default DropdownVendor