import PageHeading from "../components/PageHeading.jsx";
import {useEffect, useState} from "react";
import axios from "axios";
import EditableTextField from "../components/EditableTextField.jsx";
import DeleteConfirmation from "../components/DeleteConfirmation.jsx";
import PlusInput from "../components/PlusInput.jsx";

import {BASE_URL} from "../config.js";

function Vendors() {
    const [vendors, setVendors] = useState([]);

    // delete
    const [showDelete, setShowDelete] = useState(false)
    const [vendorId, setVendorId] = useState(0)
    const [vendorName, setVendorName] = useState("")

    useEffect(() => {
        axios
            .get(`${BASE_URL}/vendors`)
            .then(response => setVendors(response.data))
    }, []);

    return (
        <div className={"main main-spacing"}>
            <PageHeading title={"Výrobcovia"} number={vendors.length}/>
            <div className={"bigtable"}>
                <table>
                    <thead>
                    <tr>
                        <td>Názov výrobcu</td>
                        <td>Odstrániť</td>
                    </tr>
                    </thead>
                    <tbody>
                    {vendors.map((item, key) => { return (
                        <tr key={key}>
                            <td><EditableTextField
                                text={item.name_vendor}
                                type={"vendors"}
                                id={item.id_vendor}
                                field={"name_vendor"}
                            /></td>
                            <td className={"text-center align-middle"}>
                                <img
                                    src="/src/images/delete.png"
                                    alt=""
                                    className={"icon clickable"}
                                    onClick={() => {
                                        setVendorId(item.id_vendor);
                                        setVendorName(item.name_vendor);
                                        setShowDelete(true)
                                    }}
                                />
                            </td>
                        </tr>
                        )})}
                    </tbody>
                </table>
            </div>
            <PlusInput endpoint={`${BASE_URL}/vendors`} fieldName={"name_vendor"}/>
            {
                showDelete && <DeleteConfirmation
                    name={`výrobcu ${vendorName}`}
                    endpoint={`${BASE_URL}/vendors/${vendorId}`}
                    setShowDelete={setShowDelete}
                    redirect={"/vyrobcovia"}
                />
            }
        </div>
    )
}

export default Vendors