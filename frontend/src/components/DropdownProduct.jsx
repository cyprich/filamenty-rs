import {useEffect, useState} from "react";
import axios from "axios";

import {BASE_URL} from "../config.js";

function DropdownProduct({value, idFilament, setIdProduct}) {
    const [products, setProducts] = useState([])

    useEffect(() => {
        axios
            .get(`${BASE_URL}/products_full`)
            .then(response => setProducts(response.data))
    }, []);

    function handleChange(selectedItem) {
        if (idFilament != undefined) {
            axios
                .patch(`${BASE_URL}/filaments/${idFilament}`, {
                    "key": "id_product",
                    "value": selectedItem
                })
                .then(() => window.location.reload())
                .catch(error => console.error(error))
        }

        if (setIdProduct != undefined) {
            setIdProduct(selectedItem)
        }
    }

    return (
        <select
            name="productdropdown"
            value={String(value)}
            onChange={(e) => handleChange(e.target.value)}>
            {
                setIdProduct && <option value={0} disabled>Vyberte...</option>
            }
            {products.map((item, key) => { return (
                <option
                    key={key}
                    value={String(item.id_product)}>
                    {item.name_product} ({item.name_vendor})
                </option>
            )})}
        </select>
    )

}

export default DropdownProduct
