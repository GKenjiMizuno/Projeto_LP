import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import './login.css';

const Login = () => {
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const navigate = useNavigate();

  const handleSubmit = async (e) => {
    e.preventDefault();

    const response = await fetch('http://127.0.0.1:8080/api/login', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ password }),
    });

    const data = await response.json();
    if (response.ok) {
      console.log('Dados recebidos:', data); // Adicione um log para verificar a resposta
      navigate('/home', { state: { message: data.message, devicesStatus: data.devices_status, authenticated: data.authenticated, hora_atual: data.hora_atual, temp_atual: data.temp_atual} });
    } else {
      setError(data.message);
    }
  };

  return (
    <div className="body">
      <div className="container">
        <div className="container-header">
          <input
            type="password"
            placeholder="Digite a senha"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
          />
          <button onClick={handleSubmit}>Enviar</button>
          {error && <p className="error">{error}</p>}
        </div>
      </div>
    </div>
  );
};

export default Login;
