import {useEffect, useState} from "react";
import axios from "axios";

import FilamentCard from "../components/FilamentCard.jsx";
import PageHeading from "../components/PageHeading.jsx";
import PlusButton from "../components/PlusButton.jsx";

import {BASE_URL} from "../config.js";

function Filaments() {
    const [filaments, setfilaments] = useState([]);

    useEffect(() => {
        axios
            .get(`${BASE_URL}/filaments_full/`)
            .then((response) => {
                setfilaments(response.data);
            })
            .catch((error) => console.error(error));
    }, []);

    return (
        <div className={"main main-spacing"}>
            <PageHeading title={"Filamenty"} number={filaments.length}/>
            <div className={"grid grid-cols-5 gap-8 portrait:grid-cols-1 "}>
                {filaments.map((item, key) => {
                    return (
                        <FilamentCard filament={item} key={key}/>
                    );
                })}
            </div>
            <PlusButton redirect={"/filament/novy"}/>
        </div>
    );
}

export default Filaments;
