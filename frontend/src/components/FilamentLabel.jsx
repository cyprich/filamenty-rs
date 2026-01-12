import {useNavigate} from "react-router-dom";
import FilamentLabelColors from "./FilamentLabelColors.jsx";

import {BASE_URL} from "../config.js";

function FilamentLabel({filament, key}) {
    let navigate = useNavigate();

    return (
        <div
            className={"no-break flex justify-between items-center gap-8 w-max p-8 border relative portrait:scale-90"}
            key={key}>
            <div className={"flex flex-col gap-3 portrait:gap-1.5"}>
                <div className={"flex gap-3 items-center"}>
                    <FilamentLabelColors color={filament.color_hex}/>
                    <div>
                        <p className={"text-xl font-extrabold"}>
                            {filament.name_product}
                        </p>
                        <p className={"text-xl flex gap-4 justify-between leading-5"}>
                            <span>{filament.name_vendor}</span>
                            <span className={"font-extralight"}>{filament.name_material}</span>
                        </p>
                    </div>
                </div>
                <table className={"info-table"}>
                    <tbody>
                    <tr>
                        <td>Temp:</td>
                        <td>{filament.temp_min} - {filament.temp_max} °C</td>
                    </tr>
                    <tr>
                        <td>Bed:</td>
                        <td>
                            {filament.temp_bed_min}{" "}
                            {filament.temp_bed_max && `- ${filament.temp_bed_max}`}°C
                        </td>
                    </tr>
                    <tr>
                        <td>Price:</td>
                        <td>{(filament.price || 0).toFixed(2)} €</td>
                    </tr>
                    </tbody>
                </table>
            </div>
            <img
                className={"clickable-small w-32 h-32 portrait:w-24 portrait:h-24 border-6 border-white"}
                src={`${BASE_URL}/images/${filament.qr_path}`}
                onClick={() => navigate(`/filament/${filament.id_filament}`)}
                alt=""
            />
        </div>
    );
}

export default FilamentLabel