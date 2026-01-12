import {useEffect, useState} from "react";

import axios from "axios";
import FilamentLabel from "../components/FilamentLabel.jsx";
import PageTitle from "../components/PageTitle.jsx";

import {BASE_URL} from "../config.js";

function Labels() {
    const [filaments, setfilaments] = useState([]);

    useEffect(() => {
        axios
            .get(`${BASE_URL}/filaments_full/`)
            .then((response) => {
                const data = response?.data;
                if (Array.isArray(data)) {
                    setfilaments(data)
                } else {
                    setfilaments([]);
                }
            })
            .catch((error) => console.error(error));
    }, []);

    return (
        <div className={"main main-spacing labels items-center portrait:gap-8 portrait:!px-8"}>
            <PageTitle title={"Štítky"} number={filaments.length}/>
            <div className={"flex flex-wrap gap-8 portrait:flex-col portrait:items-center"}>
                {filaments.map((filament, key) => {
                    return <div key={key}>
                        <FilamentLabel filament={filament}/>
                    </div>
                })}
            </div>
            <p
                className={"no-print clickable-small custom-border border cursor-pointer px-16 py-5 shadow-lg bg-zinc-50 text-center portrait:-my-0 portrait:w-full"}
                onClick={() => window.print()}>
                Tlačiť
            </p>
            <p className={"no-print text-[0.9rem] text-center"}>
                Pre optimálny výsledok je potrebné{" "}
                <span className={"font-semibold"}>zapnúť tlačenie pozadia </span>
                (Print Backgrounds)
            </p>
        </div>
    );
}

export default Labels;
