import {useState} from "react";
import axios from "axios";

import {BASE_URL} from "../config.js";
import editIcon from "../images/edit.png";
import checkIcon from "../images/check.png";
import crossIcon from "../images/cross.png";

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
                        <p className={"whitespace-nowrap"}>{value} {additionalText}</p>
                        <img
                            src={editIcon}
                            alt="edit"
                            className={"clickable h-6 w-6 opacity-0 group-hover:opacity-100 portrait:opacity-100"}
                            onClick={() => {setEditing(true)}}>
                        </img>
                    </>
                ) : (
                    <>
                        <input
                            className={"border rounded-xl px-2 py-1 portrait:max-w-48"}
                            type="text"
                            value={value}
                            onChange={(e) => setValue(e.target.value)}
                        />
                        <img
                            className={"clickable h-7 w-7 -mr-2"}
                            src={checkIcon}
                            alt="confirm"
                            onClick={() => handleConfirm()}
                        />
                        <img
                            className={"clickable h-7 w-7"}
                            src={crossIcon}
                            alt="cancel"
                            onClick={() => { setEditing(false) ; setValue(text) }}
                        />
                    </>
                )
            }
        </div>
    )
}

export default EditableTextField