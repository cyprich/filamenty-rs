import {useState} from "react";
import axios from "axios";

import {BASE_URL} from "../config.js";

function FilamentEditableColor({idFilament, colorHex}) {
    const [color, setColor] = useState(colorHex);

    function handleChange(color) {
        axios
            .patch(`${BASE_URL}/filaments/${idFilament}`, {
                "key": "color_hex",
                "value": color
            })
            .then(() => window.location.reload())
            .catch(error => console.error(error))
    }

    return (
        <input
            type={"color"}
            value={color}
            className={"w-10 h-10 clickable"}
            style={{background: `${colorHex}`}}
            onChange={(e) => {
                setColor(e.target.value)
                handleChange(e.target.value)
            }}
        />
    )
}

export default FilamentEditableColor