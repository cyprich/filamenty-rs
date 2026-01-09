import {useEffect, useState} from "react";
import axios from "axios";
import PageHeading from "../components/PageHeading.jsx";
import EditableTextField from "../components/EditableTextField.jsx";
import DeleteConfirmation from "../components/DeleteConfirmation.jsx";
import PlusInput from "../components/PlusInput.jsx";

import {BASE_URL} from "../config.js";

function Materials() {
    const [materials, setMaterials] = useState([])

    // delete
    const [showDelete, setShowDelete] = useState(false)
    const [materialId, setMaterialId] = useState(0)
    const [materialName, setMaterialName] = useState("")

    useEffect(() => {
        axios
            .get(`${BASE_URL}/materials`)
            .then(response => setMaterials(response.data))
    }, []);

    return (
        <div className={"main main-spacing"}>
            <PageHeading title={"Materiály"} number={materials.length}/>
            <div className={"bigtable"}>
                <table>
                    <thead><tr>
                        <td>Názov materiálu</td>
                        <td>Odstrániť</td>
                    </tr></thead>
                    <tbody>
                    {
                        materials.map((item, key) => { return (
                            <tr key={key}>
                                <td><EditableTextField
                                    text={item.name_material}
                                    type={"materials"}
                                    id={item.id_material}
                                    field={"name_material"}
                                /></td>
                                <td>
                                    <img
                                        src="/src/images/delete.png"
                                        alt=""
                                        className={"icon clickable"}
                                        onClick={() => {
                                            setMaterialId(item.id_material);
                                            setMaterialName(item.name_material);
                                            setShowDelete(true)
                                        }}
                                    />
                                </td>
                            </tr>
                        )})
                    }
                    </tbody>
                </table>
            </div>
            <PlusInput endpoint={`${BASE_URL}/materials`} fieldName={"name_material"}/>
            {
                showDelete && <DeleteConfirmation
                    name={`materiál ${materialName}`}
                    endpoint={`${BASE_URL}/materials/${materialId}`}
                    setShowDelete={setShowDelete}
                    redirect={"/materialy"}
                />
            }
        </div>
    )
}

export default Materials