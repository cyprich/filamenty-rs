import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import axios from "axios";

import DeleteConfirmation from "../components/DeleteConfirmation.jsx";
import EditableTextField from "../components/EditableTextField.jsx";
import DropdownProduct from "../components/DropdownProduct.jsx";
import FilamentEditableColor from "../components/FilamentEditableColor.jsx";
import EditableImage from "../components/EditableImage.jsx";
import FormattedDate from "../components/FormattedDate.jsx";

import {BASE_URL} from "../config.js";
import deleteRedIcon from "../images/delete_red.png";

function Filament() {
    const { id } = useParams();

    const [filament, setFilament] = useState(null);
    const [responseCode, setResponseCode] = useState(0);

    const [showDelete, _setShowDelete] = useState(false);

    useEffect(() => {
        axios
            .get(`${BASE_URL}/filaments/${id}/`)
            .then(response => {
                setFilament(response.data);
                setResponseCode(response.status);
            })
            .catch(error => console.error(error));
    }, [id]);

    function setShowDelete(state) {
        _setShowDelete(state);
    }

    return (
        <div className={"main filament flex flex-col gap-8 portrait:!gap-6"}>
            {responseCode === 200 || responseCode === 201 ? (
                <>
                    <div className={"flex items-center gap-6 portrait:justify-between"}>
                        <img
                            className={"h-32"}
                            src={`${BASE_URL}/images/${filament.qr_path}`}
                            alt=""
                        />
                        <div className={"portrait:text-right"}>
                            <div className={"flex flex-col portrait:justify-end"}>
                                <h2 className={"!text-3xl font-bold portrait:!text-lg portrait:-ml-8"}>
                                    <DropdownProduct value={filament.id_product} idFilament={filament.id_filament}/>
                                </h2>
                                <div className={"flex items-center gap-2 pl-2 portrait:-ml-8"}>
                                    <FilamentEditableColor idFilament={filament.id_filament} colorHex={filament.color_hex}/>
                                    <div className={"font-medium text-xl"}>
                                        <EditableTextField
                                            text={filament.color_name}
                                            type={"filaments"}
                                            id={filament.id_filament}
                                            field={"color_name"}
                                        />
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <p className={"-mt-6"}>Naposledy upravené: <FormattedDate date={filament.last_update}/></p>
                    <div
                        style={{background: `${filament.color_hex}`}}
                        className={"w-full h-1 rounded-full portrait:-mt-0"}
                    />
                    <div className={"flex gap-16 portrait:flex-col portrait:items-center"}>
                        <EditableImage
                            defaultSrc={`${BASE_URL}/images/${filament.image_path}`}
                            idFilament={filament.id_filament}/>
                        <div className={"w-full flex flex-col gap-4"}>
                            <div className={"flex flex-col gap-3 portrait:items-center"}>
                                <h2>Informácie</h2>
                                <table className={"info-table w-max"}>
                                    <tbody>
                                    <tr>
                                        <td>{filament.temp_max ? "Minimálna t" : "T"}eplota tlače</td>
                                        <td>
                                            <EditableTextField
                                                text={filament.temp_min}
                                                type={"products"}
                                                id={filament.id_product}
                                                field={"temp_min"}
                                                additionalText={"°C"}
                                            />
                                        </td>
                                    </tr>
                                    {
                                        filament.temp_max &&
                                            <tr>
                                                <td>Maximálna teplota tlače</td>
                                                <td>
                                                    <EditableTextField
                                                        text={filament.temp_max}
                                                        type={"products"}
                                                        id={filament.id_product}
                                                        field={"temp_max"}
                                                        additionalText={"°C"}
                                                    />
                                                </td>
                                            </tr>
                                    }
                                    <tr>
                                        <td>
                                            {filament.temp_bed_max ? "Minimálna t" : "T"}eplota podložky
                                        </td>
                                        <td>
                                            <EditableTextField
                                                text={filament.temp_bed_min}
                                                type={"products"}
                                                id={filament.id_product}
                                                field={"temp_bed_min"}
                                                additionalText={"°C"}
                                            />
                                        </td>
                                    </tr>
                                    {filament.temp_bed_max && (
                                        <tr>
                                            <td>Maximálna teplota podložky</td>
                                            <td>
                                                <EditableTextField
                                                    text={filament.temp_bed_max}
                                                    type={"products"}
                                                    id={filament.id_product}
                                                    field={"temp_bed_max"}
                                                    additionalText={"°C"}
                                                />
                                            </td>
                                        </tr>
                                    )}
                                    <tr>
                                        <td><b>Zostávajúca hmotnosť</b></td>
                                        <td>
                                            <b><EditableTextField
                                                text={filament.netto_weight}
                                                type={"filaments"}
                                                id={filament.id_filament}
                                                field={"netto_weight"}
                                                additionalText={"g"}
                                            /></b>
                                        </td>
                                    </tr>
                                    <tr>
                                        <td>Hmotnosť so spoolom</td>
                                        <td>
                                            <EditableTextField
                                                text={filament.netto_weight + filament.spool_weight}
                                                type={"filaments"}
                                                id={filament.id_filament}
                                                field={"brutto_weight"}
                                                additionalText={"g"}
                                                additionalData={filament.spool_weight}
                                            />
                                        </td>
                                    </tr>
                                    <tr>
                                        <td>Hmotnosť spoolu</td>
                                        <td>
                                            <EditableTextField
                                                text={filament.spool_weight}
                                                type={"filaments"}
                                                id={filament.id_filament}
                                                field={"spool_weight"}
                                                additionalText={"g"}
                                            />
                                        </td>
                                    </tr>
                                    <tr>
                                        <td>Pôvodná hmotnosť</td>
                                        <td>
                                            <EditableTextField
                                                text={filament.original_weight}
                                                type={"filaments"}
                                                id={filament.id_filament}
                                                field={"original_weight"}
                                                additionalText={"g"}
                                            />
                                        </td>
                                    </tr>
                                    <tr>
                                        <td>Cena</td>
                                        <td>
                                            <EditableTextField
                                                text={filament.price}
                                                type={"filaments"}
                                                id={filament.id_filament}
                                                field={"price"}
                                                additionalText={"€"}
                                            />
                                        </td>
                                    </tr>
                                    </tbody>
                                    <tfoot>
                                    <tr className={"portrait:!hidden"}>
                                        <td
                                            className={
                                                "flex mt-4 justify-center items-center cursor-pointer border rounded-lg border-red-600 hover:bg-red-600/10 select-none"
                                            }
                                            onClick={() => {
                                                setShowDelete(true);
                                            }}
                                        >
                                            <img
                                                className={"w-8 -ml-2"}
                                                src={deleteRedIcon}
                                                alt=""
                                            />
                                            <p>Odstrániť</p>
                                        </td>
                                    </tr>
                                    </tfoot>
                                </table>
                                <div
                                    className={
                                        "hidden portrait:flex justify-center items-center gap-2 w-full border border-red-600 py-4 rounded-xl hover:bg-red-600/10 select-none"
                                    }
                                    onClick={() => {
                                        setShowDelete(true);
                                    }}
                                >
                                    <img
                                        className={"w-8"}
                                        src={deleteRedIcon}
                                        alt=""
                                    />
                                    <p>Odstrániť</p>
                                </div>
                            </div>
                            {showDelete && (
                                <DeleteConfirmation
                                    name={`filament ${filament.name_vendor} ${filament.name_product} (${filament.color_name})`}
                                    endpoint={`${BASE_URL}/filaments/${filament.id_filament}`}
                                    image_url={`${BASE_URL}/images/filament${filament.id_filament}.png`}
                                    setShowDelete={setShowDelete}
                                />
                            )}
                        </div>
                    </div>
                </>
            ) : (
                <div className={"flex flex-col justify-center gap-3 pt-32 text-center"}>
                    <h2 className={"!text-4xl font-bold"}>Filament sa nenašiel</h2>
                    <h2 className={"!text-3xl pb-2 font-semibold"}>:(</h2>
                    <p>Pravdepodobne ste zadali zlé číslo filamentu</p>
                </div>
            )}
        </div>
    );
}

export default Filament;
