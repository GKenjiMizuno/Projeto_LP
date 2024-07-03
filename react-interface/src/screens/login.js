import React from "react";
import './login.css'
import { Link } from "react-router-dom";

const Login = () => {


    return (
      <div className="body">
        <div className = 'container'>
            <div className="container-header">
                <input id = 'password' name = 'password' type="password" placeholder="Digite a senha"/>
                <Link to = '/home'>
                  <button className="button-enviar">Enviar</button>
                </Link>
            </div>
          
        </div>
      </div>  

    )


}

export default Login