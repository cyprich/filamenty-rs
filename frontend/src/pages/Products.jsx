import {useEffect, useState} from "react";
import axios from "axios";
import PageTitle from "../components/PageTitle.jsx";
import EditableTextField from "../components/EditableTextField.jsx";
import PlusButton from "../components/PlusButton.jsx";
import DropdownVendor from "../components/DropdownVendor.jsx";
import DropdownMaterial from "../components/DropdownMaterial.jsx";
import DeleteConfirmation from "../components/DeleteConfirmation.jsx";

import {BASE_URL} from "../config.js";
import deleteIcon from "../images/delete.png";

function Products() {
    const [products, setProducts] = useState([]);

    // delete
    const [showDelete, setShowDelete] = useState(false);
    const [productId, setProductId] = useState(0)
    const [productName, setProductName] = useState("")

    useEffect(() => {
        axios
            .get(`${BASE_URL}/products_full`)
            .then(response => {
                const data = response?.data;
                if (Array.isArray(data)) {
                    setProducts(data)
                } else {
                    setProducts([]);
                }
            })
            .catch(error => console.error(error))
    }, [setProducts]);

    return (
        <div className={"main main-spacing"}>
            <PageTitle title={"Produkty"} number={products.length}/>
            <div className={"overflow-hidden w-full bigtable"}>
                <table className={"products-table"}>
                    <thead>
                    <tr>
                        <td>Názov produktu</td>
                        <td>Výrobca</td>
                        <td>Materiál</td>
                        <td>Teplota tlače</td>
                        <td>Teplota podložky</td>
                        <td>Priemer</td>
                        <td>Odstrániť</td>
                    </tr>
                    </thead>
                    <tbody>
                        {products.map((item, key) => {return (
                            <tr key={key}>
                                <td><EditableTextField
                                    text={item.name_product}
                                    type={"products"}
                                    id={item.id_product}
                                    field={"name_product"}
                                /></td>
                                <td><DropdownVendor value={item.id_vendor} idProduct={item.id_product}/></td>
                                <td><DropdownMaterial value={item.id_material} idProduct={item.id_product}/></td>
                                <td className={"temp-td"}>
                                    <div>
                                        Od
                                        <EditableTextField
                                            text={item.temp_min}
                                            additionalText={"°C"}
                                            type={"products"}
                                            id={item.id_product}
                                            field={"temp_min"}
                                        />
                                    </div>
                                    <div>
                                        Do
                                        <EditableTextField
                                            text={item.temp_max || "-"}
                                            additionalText={"°C"}
                                            type={"products"}
                                            id={item.id_product}
                                            field={"temp_max"}
                                        />
                                    </div>
                                </td>
                                <td className={"temp-td"}>
                                    <div>
                                        Od
                                        <EditableTextField
                                            text={item.temp_bed_min}
                                            additionalText={"°C"}
                                            type={"products"}
                                            id={item.id_product}
                                            field={"temp_bed_min"}
                                        />
                                    </div>
                                    <div>
                                        Do
                                        <EditableTextField
                                            text={item.temp_bed_max || "-"}
                                            additionalText={"°C"}
                                            type={"products"}
                                            id={item.id_product}
                                            field={"temp_bed_max"}
                                        />
                                    </div>
                                </td>
                                <td><EditableTextField
                                    text={item.diameter}
                                    additionalText={"mm"}
                                    type={"products"}
                                    id={item.id_product}
                                    field={"diameter"}
                                /></td>
                                <td>
                                    <img
                                        src={deleteIcon}
                                        alt=""
                                        className={"icon clickable"}
                                        onClick={() => {
                                            setProductId(item.id_product);
                                            setProductName(item.name_product);
                                            setShowDelete(true)
                                        }}
                                    />
                                </td>
                            </tr>
                            )})}
                    </tbody>
                </table>
            </div>
            <PlusButton redirect={"/produkt/novy"}/>
            {
                showDelete && <DeleteConfirmation
                    name={`produkt ${productName}`}
                    endpoint={`${BASE_URL}/products/${productId}`}
                    setShowDelete={setShowDelete}
                    redirect={"/produkty"}
                />
            }
        </div>
    )
}

export default Products