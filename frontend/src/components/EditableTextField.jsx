import {useState} from "react";
import axios from "axios";

import {BASE_URL} from "../config.js";

function EditableTextField({text, additionalText, type, id, field, additionalData}) {
    const [editing, setEditing] = useState(false)
    const [value, setValue] = useState(text)

    function handleConfirm() {
        let local_value = value
        let local_field = field
        // special case - brutto_weight
        if (field === "brutto_weight") {
            if (additionalData === undefined) {
                console.error("Undefined `additionalData` when specifying `brutto_weight`")
                return
            }

            local_field = "netto_weight"
            local_value = value - additionalData
        }

        axios
            .patch(`${BASE_URL}/${type}/${id}`, {
                key: local_field,
                value: String(local_value)
            })
            .then(() => window.location.reload())
            .catch((error) => console.error(error))
    }

    return (
        <div className={"flex items-center gap-4 group"}>
            {
                !editing ? (
                    <>
                        <p>{value} {additionalText}</p>
                        <img
                            src="/src/images/edit.png"
                            className={"clickable h-6 w-6 opacity-0 group-hover:opacity-100"}
                            onClick={() => {setEditing(true)}}>
                        </img>
                    </>
                ) : (
                    <>
                        <input
                            className={"border rounded-xl px-2 py-1"}
                            type="text"
                            value={value}
                            onChange={(e) => setValue(e.target.value)}
                        />
                        <img
                            className={"clickable h-7 w-7 -mr-2"}
                            src="/src/images/check.png"
                            onClick={() => handleConfirm()}
                        />
                        <img
                            className={"clickable h-7 w-7"}
                            src="/src/images/cross.png"
                            onClick={() => { setEditing(false) ; setValue(text) }}
                        />
                    </>
                )
            }
        </div>
    )
}

export default EditableTextField