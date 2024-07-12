import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import './register.css';

const Register = () => {
  const [masterPassword, setMasterPassword] = useState('');
  const [newPassword, setNewPassword] = useState('');
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');
  const navigate = useNavigate();

  const handleSubmit = async (e) => {
    e.preventDefault();

    const response = await fetch('http://127.0.0.1:8080/api/register', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ master_password: masterPassword, new_password: newPassword }),
    });

    const data = await response.json();
    if (response.ok) {
      setSuccess(data.message);
      setError('');
    } else {
      setError(data.message);
      setSuccess('');
    }
  };

  return (
    <div className="container">
      <div className="container-header-register">
        <h1>Registrar Nova Senha</h1>
      </div>
      <div className='container-body'>
        <input
          type="password"
          placeholder="Digite a senha mestre"
          value={masterPassword}
          onChange={(e) => setMasterPassword(e.target.value)}
        />
        <input
          type="password"
          placeholder="Digite a nova senha"
          value={newPassword}
          onChange={(e) => setNewPassword(e.target.value)}
        />
        <button onClick={handleSubmit}>Registrar</button>
        {error && <p className="error">{error}</p>}
        {success && <p className="success">{success}</p>}
        <button onClick={() => navigate('/')}>Voltar ao Login</button>
      </div>
    </div>
  );
};

export default Register;
