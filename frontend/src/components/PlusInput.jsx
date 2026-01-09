import {useState} from "react";
import axios from "axios";

function PlusInput({fieldName, endpoint}) {
    const [value, setValue] = useState("")

    function handleConfirm() {
        if (!value) return;

        axios
            .post(endpoint, { [fieldName]: value })
            .then(() => window.location.reload())
            .catch(error => {
                if (error.response.data.includes("already exists")) {
                    alert("Could not add - this item already exists")
                }
            })
    }

    return (
        <div className={"flex gap-2 ml-2"}>
            <input
                type="text"
                placeholder={"Pridať..."}
                className={"border border-black p-2 rounded-xl"}
                onChange={(e) => setValue(e.target.value)}
            />
            <button
                onClick={() => handleConfirm()}
                className={"clickable-small"}
            >Potvrdiť</button>
        </div>
    )
}

export default PlusInput