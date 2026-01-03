import {BrowserRouter as Router, Routes, Route} from "react-router-dom";

import Header from "./Header.jsx";
import Footer from "./Footer.jsx";
import Filaments from "./Filaments.jsx";
import Filament from "./Filament.jsx";
import AddFilament from "./AddFilament.jsx"
import Labels from "./Labels"

function App() {
    return (
        <Router>
            <Header/>
            <Routes>
                    <Route path={"/"} element={<Filaments/>}/>
                    <Route path={"/novy"} element={<AddFilament/>}/>
                    <Route path={"/stitky"} element={<Labels/>}/>
                    <Route path={"/filament/:id"} element={<Filament/>}/>

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
