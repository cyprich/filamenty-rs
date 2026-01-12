import {useNavigate} from "react-router-dom";
import {useState} from "react";
import iconImg from "./images/icon.png";

function Header() {
    let navigate = useNavigate();
    const [showMenu, setShowMenu] = useState(false);

    return (
        <>
            <header className={"portrait:hidden flex items-center gap-8 px-16"}>
                <img className={"clickable-small w-12 h-12 mr-4"}
                     src={iconImg} alt=""
                     onClick={() => navigate("/")}/>
                <Links navigate={navigate} setShowMenu={setShowMenu}/>
            </header>
            <header className={"hidden portrait:flex justify-between px-8"}>
                <img className={"clickable-small w-12 h-12 mr-4"}
                     src={iconImg} alt=""
                     onClick={() => navigate("/")}/>
                <p
                    className={"text-2xl clickable-small"}
                    onClick={() => setShowMenu(true)}
                >☰</p>
                { showMenu &&
                    <div
                        className={"absolute top-0 right-0 bg-zinc-900 w-[80vw] h-screen p-4 z-10"}>
                        <div className={"flex justify-between"}>
                            <p className={"font-extrabold text-lg"}>Menu</p>
                            <p className={"font-bold text-lg"} onClick={() => setShowMenu(false)}>✖</p>
                        </div>
                        <Links navigate={navigate} setShowMenu={setShowMenu}/>
                    </div>
                }
            </header>
        </>
    )
}

function Links({navigate, setShowMenu}) {
    return (
        <>
            <p className={"clickable-small"} onClick={() => {
                navigate("/");
                setShowMenu(false)
            }}>Filamenty</p>
            <p className={"clickable-small"} onClick={() => {
                navigate("/produkty");
                setShowMenu(false)
            }}>Produkty</p>
            <p className={"clickable-small"} onClick={() => {
                navigate("/vyrobcovia");
                setShowMenu(false)
            }}>Výrobcovia</p>
            <p className={"clickable-small"} onClick={() => {
                navigate("/materialy");
                setShowMenu(false)
            }}>Materiály</p>
            <p className={"clickable-small"} onClick={() => {
                navigate("/stitky");
                setShowMenu(false)
            }}>Štítky</p>
        </>
    )
}

export default Header;
