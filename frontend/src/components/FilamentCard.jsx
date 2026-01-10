/* eslint-disable react/prop-types */
import {useNavigate} from "react-router-dom";
import {useState} from "react";
import DeleteConfirmation from "./DeleteConfirmation.jsx";
import FilamentCardBar from "./FilamentCardBar.jsx";

import {BASE_URL} from "../config.js";
import editIcon from "../images/edit.png";
import deleteIcon from "../images/delete.png";

function FilamentCard({filament}) {
    const navigate = useNavigate();

    const [showDelete, setShowDelete] = useState(false)

    return (
        <div
            className={`flex flex-col gap-5 items-center p-6 custom-border 
            ${filament.netto_weight <= 0 ? "opacity-40 grayscale-25 border-none shadow-none!" : "border"}`}>
            <img
                className={"clickable-small object-contain"}
                src={`${BASE_URL}/images/${filament.image_path}`}
                alt=""
                onClick={() => navigate(`/filament/${filament.id_filament}`)}
            />
            <div className={"w-full flex flex-col items-center gap-2"}>
                <p>{filament.netto_weight} g left</p>
                <FilamentCardBar
                    colorHex={filament.color_hex}
                    colorName={filament.color_name}
                    nettoWeight={filament.netto_weight}
                    originalWeight={filament.original_weight}/>
            </div>
            <div className={"w-full flex flex-col gap-2"}>
                <div className={"flex justify-between"}>
                    <div className={"flex flex-col"}>
                        <h2 className={"font-bold"}>{filament.name_product}</h2>
                        <h2 className={"font-semibold"}>{filament.name_vendor}</h2>
                    </div>
                    <h2 className={"font-extralight self-end"}>{filament.name_material}</h2>
                </div>
                <div className={"flex justify-between"}>
                    <table className={"info-table"}>
                        <tbody>
                            <tr>
                                <td>Teplota</td>
                                <td>: {filament.temp_min} - {filament.temp_max} °C</td>
                            </tr>
                            <tr>
                                <td>Podložka</td>
                                <td>
                                    : {filament.temp_bed_min}{" "}
                                    {filament.temp_bed_max ? `- ${filament.temp_bed_max}` : ""} °C
                                </td>
                            </tr>
                            <tr>
                                <td>Cena</td>
                                <td>: {(filament.price).toFixed(2)} €</td>
                            </tr>
                        </tbody>
                    </table>
                    <div className={"flex flex-col justify-end gap-2"}>
                        <img
                            className={"icon clickable"}
                            src={editIcon}
                            alt="edit"
                            onClick={() => navigate(`/filament/${filament.id_filament}`)}
                        />
                        <img
                            className={"icon clickable"}
                            src={deleteIcon}
                            alt="delete"
                            onClick={() => setShowDelete(true)}
                        />
                    </div>
                </div>
            </div>
            {
                showDelete &&
                <DeleteConfirmation
                    setShowDelete={setShowDelete}
                    image_url={`${BASE_URL}/images/${filament.image_path}`}
                    endpoint={`${BASE_URL}/filaments/${filament.id_filament}`}
                    redirect={"/"}
                />
            }
        </div>
    );
}

export default FilamentCard;