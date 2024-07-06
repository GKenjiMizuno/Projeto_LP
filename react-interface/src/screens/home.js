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


  const createStatusMessage = (status) => {
    return `Luz: ${status.luz ? 'Ligada' : 'Desligada'}, 
            Tranca: ${status.tranca ? 'Trancada' : 'Destrancada'}, 
            Alarme: ${status.alarme ? 'Ativado' : 'Desativado'}, 
            Cortinas: ${status.cortinas ? 'Abertas' : 'Fechadas'}, 
            Robo: ${status.robo ? 'Ligado' : 'Desligado'}, 
            Cafeteira: ${status.cafeteira ? 'Ligada' : 'Desligada'}, 
            Ar Condicionado: ${status.ar_condicionado ? 'Ligado' : 'Desligado'}, 
            Aquecedor: ${status.aquecedor ? 'Ligado' : 'Desligado'},`;
  };

  const statusMessage = createStatusMessage(devicesStatus);

  return (
    <div className="body">
      <div className="container">
        <div className="container-header">
          <p>{statusMessage}</p>
        </div>
      </div>
    </div>
  );
};

export default Home;
