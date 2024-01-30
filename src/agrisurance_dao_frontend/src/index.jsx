import * as React from "react";
import REACTDOM from "react-dom";
import App from "./App"
import { agrisurance_dao_backend } from "../../declarations/agrisurance_dao_backend";

const Root = () => {
    return(
        <App />
    )
}

REACTDOM.render(<Root/>, document.getElementById("app"));
