import {useNavigate} from "react-router-dom";

function Header() {
    let navigate = useNavigate();

    return (
        <header
            className={"flex items-center gap-8 px-16 py-5 bg-zinc-950 portrait:grid portrait:grid-cols-3 portrait:px-8"}>
            <img className={"clickable-small w-12 h-12 mr-4 portrait:mr-0 portrait:order-2 justify-self-center"}
                 src={"/src/images/icon.png"} alt=""
                 onClick={() => navigate("/")}/>
            <p className={"clickable-small portrait:order-1 justify-self-center"}
               onClick={() => navigate("/")}>Filamenty</p>
            <p className={"clickable-small portrait:order-3 justify-self-center"}
               onClick={() => navigate("/stitky")}>Štítky</p>
        </header>
    )
}

export default Header;