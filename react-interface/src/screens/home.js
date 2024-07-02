import React from "react";
import './home.css';
import DataFetcher from "../DataFetcher";

const Home = () => {

    return (
        <div className="body">
            <div className="container" >
                <div className="container-header">
                    <h1>Dispositivos</h1>
                </div>
                <DataFetcher />
            </div>
        </div>
    )


    
}

export default Home