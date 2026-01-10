import {BrowserRouter as Router, Routes, Route} from "react-router-dom";

import Header from "./Header.jsx";
import Footer from "./Footer.jsx";
import Filaments from "./pages/Filaments.jsx";
import Filament from "./pages/Filament.jsx";
import NewFilament from "./pages/NewFilament.jsx"
import Labels from "./pages/Labels.jsx"
import Products from "./pages/Products.jsx";
import Vendors from "./pages/Vendors.jsx";
import Materials from "./pages/Materials.jsx";
import NewProduct from "./pages/NewProduct.jsx";

import {BASE_URL} from "./config.js";

function App() {

    console.log("BASE_URL", BASE_URL);

    return (
        <Router>
            <Header/>
            <Routes>
                <Route path={"/"} element={<Filaments/>}/>
                <Route path={"/filament/:id"} element={<Filament/>}/>
                <Route path={"/filament/novy"} element={<NewFilament/>}/>
                <Route path={"/produkty"} element={<Products/>}/>
                <Route path={"/produkt/novy"} element={<NewProduct/>}/>
                <Route path={"/vyrobcovia"} element={<Vendors/>}/>
                <Route path={"/materialy"} element={<Materials/>}/>
                <Route path={"/stitky"} element={<Labels/>}/>

                <Route path={"*"} element={<div
                    style={{
                        minHeight: "90vh",
                        display: "flex",
                        flexDirection: "column",
                        justifyContent: "center",
                        alignItems: "center",
                        gap: "0.5em"
                    }}>
                    <h1>Stránka sa nenašla</h1>
                    <h2 style={{fontWeight: 400}}>:(</h2>
                </div>}>
                </Route>
            </Routes>
            <Footer/>
        </Router>
    );
}

export default App;
