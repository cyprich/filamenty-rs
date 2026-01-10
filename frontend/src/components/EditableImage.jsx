import {useState, useEffect} from "react";
import axios from "axios";

import {BASE_URL} from "../config.js";
import missingFilament from "../images/missing_filament.png";

function EditableImage({defaultSrc, idFilament, setImagePath}) {
    const [imagePreview, setImagePreview] = useState(defaultSrc);

    function handleChange(e) {
        const selectedFile = e.target.files[0];
        if (!selectedFile) return;

        setImagePreview(URL.createObjectURL(selectedFile) ?? missingFilament);

        const formData = new FormData();
        formData.append("file", selectedFile);

        axios
            .post(`${BASE_URL}/images`, formData)
            .then((response) => {
                const filename = response.data;

                if (idFilament !== undefined) {
                    axios
                        .patch(`${BASE_URL}/filaments/${idFilament}`, {
                            "key": "image_path",
                            "value": filename
                        })
                        .catch(error => console.error(error))
                }

                if (setImagePath !== undefined) {
                    setImagePath(filename)
                }
            })
            .catch(error => console.error(error))
    }


    return (
        <div className={"relative w-max h-max"}>
            <label htmlFor="fileInputElement">
                <img
                    src={imagePreview || missingFilament}
                    alt="filament"
                    onError={(e) => {
                        e.currentTarget.onerror = null;
                        e.currentTarget.src = missingFilament
                    }}
                    className={"clickable-small border custom-border max-w-128 max-h-128 object-contain p-8"}
                />
            </label>
            <input
                id={"fileInputElement"}
                type="file"
                accept={".jpg,.jpeg,.png,.webp"}
                className={"hidden"}
                onChange={(e) => handleChange(e)}
            />
        </div>
    )
}

export default EditableImage