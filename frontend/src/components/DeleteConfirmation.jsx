import axios from "axios";
import {useEffect} from "react";

import {useNavigate} from "react-router-dom";

function DeleteConfirmation({name, image_url, endpoint, setShowDelete, redirect}) {
    let navigate = useNavigate();

    useEffect(() => {
        document.body.style.overflow = "hidden";

        return () => {
            document.body.style.overflow = "auto";
        };
    }, []);

    function handleDelete() {
        axios
            .delete(endpoint)
            .then(() => {
                navigate(redirect);
                window.location.reload();
            })
            .catch((error) => {
                if (error.response.data.includes("referential integrity")) {
                    alert("Could not delete - this item is referenced somewhere else")
                }
            });
    }

    return (
        <div
            className={"fixed top-0 left-0 flex justify-center items-center w-screen h-screen bg-black/50 backdrop-blur-xs"}>
            <div
                className={"custom-border flex flex-col justify-center items-center gap-4 p-8 bg-zinc-100 outline outline-zinc-100 outline-offset-16 portrait:w-[80%] portrait:p-4"}>
                <h2 className={"font-semibold"}>Odstrániť?</h2>
                { image_url && <img src={image_url} alt="" className={"w-64 portrait:w-48"}/> }
                <div style={{textAlign: "center"}}>
                    <p>Naozaj si prajete odstrániť <span className={"font-medium"}>{name}</span>?</p>
                    <p>Tento krok sa nedá vrátiť späť</p>
                </div>
                {/*<div style={{ display: "flex", gap: "1em" }}>*/}
                <div className={"flex gap-1"}>
                    <button
                        className={"clickable-small border-2 border-red-600/25 hover:border-red-600 portrait:border-red-600/80"}
                        onClick={() => handleDelete()}>
                        Odstrániť
                    </button>
                    <button
                        className={"clickable-small"}
                        onClick={() => setShowDelete(false)}>
                        Zrušiť
                    </button>
                </div>
            </div>
        </div>
    );
}

export default DeleteConfirmation;
