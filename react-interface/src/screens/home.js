import React, { useEffect } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import './home.css';

const Home = () => {
  const location = useLocation();
  const navigate = useNavigate();

  useEffect(() => {
    if (!location.state) {
      navigate('/');
    }
  }, [location, navigate]);

  if (!location.state) {
    return null; // ou renderize um carregando enquanto redireciona
  }

  const { message, devicesStatus } = location.state;

  return (
    <div className="body">
      <div className="container">
        <div className="container-header">
          <h1>{message}</h1>
          <p>{devicesStatus}</p>
        </div>
      </div>
    </div>
  );
};

export default Home;
