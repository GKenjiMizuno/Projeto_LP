import React, { useEffect, useState } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import './home.css';

const Home = () => {
  const location = useLocation();
  const navigate = useNavigate();
  const [devicesStatus, setDevicesStatus] = useState(location.state?.devicesStatus || {});
  const [horaAtual, setHoraAtual] = useState(location.state?.hora_atual || 0 );
  const [tempAtual, setTempAtual] = useState(location.state?.temp_atual || 12)

  useEffect(() => {
    if (!location.state) {
      navigate('/');
    }

    const fetchData = async () => {
      try {
        const response = await fetch('http://127.0.0.1:8080/api/data');
        const data = await response.json();
        console.log('Dados recebidos:', data); // Log para verificar a resposta
        setDevicesStatus(data.devices_status);
        setHoraAtual(data.hora_atual);
        setTempAtual(data.temp_atual);
      } catch (error) {
        console.error('Erro ao buscar dados:', error);
      }
    };
    
    fetchData(); // Chamada inicial para buscar os dados assim que o componente monta
    const interval = setInterval(fetchData, 5000); // Atualiza a cada 5 segundos

    return () => clearInterval(interval); // Limpa o intervalo quando o componente desmonta
  }, [location, navigate]);

  if (!location.state) {
    return null; // ou renderize um carregando enquanto redireciona
  }

  const createStatusMessage = (status) => {
    return (
      <div className="status-grid">
        <div className="status-item">Luz: {status.luz ? 'Ligada' : 'Desligada'}</div>
        <div className="status-item">Tranca: {status.tranca ? 'Trancada' : 'Destrancada'}</div>
        <div className="status-item">Alarme: {status.alarme ? 'Ativado' : 'Desativado'}</div>
        <div className="status-item">Cortinas: {status.cortinas ? 'Abertas' : 'Fechadas'}</div>
        <div className="status-item">Robo: {status.robo ? 'Ligado' : 'Desligado'}</div>
        <div className="status-item">Cafeteira: {status.cafeteira ? 'Ligada' : 'Desligada'}</div>
        <div className="status-item">Ar Condicionado: {status.ar_condicionado ? 'Ligado' : 'Desligado'}</div>
        <div className="status-item">Aquecedor: {status.aquecedor ? 'Ligado' : 'Desligado'}</div>
      </div>
    );
  };



  const createClockMessage = (hourData) => {
    return (
      <div className='status-grid'>
        <div className='status-item'>Hora Atual: {hourData}</div>
      </div>
    );
  };
  

  const createTemperatureMessage = (temp) => {
    const temperature = Number(temp);
    return (
      <div className='status-grid'>
        <div className='status-item'>
          Temperatura Atual: {isNaN(temperature) ? 'Valor inválido' : temperature.toFixed(2)}
        </div>
      </div>
    );
  };


  const statusMessage = createStatusMessage(devicesStatus);
  const clockMessage = createClockMessage(horaAtual);
  const tempMessage  = createTemperatureMessage(tempAtual);

  return (
    <div className="body">
      <div className="container">
        <div className="container-header">
          <p>{statusMessage}</p>
          <p>{clockMessage}</p>
          <p>{tempMessage}</p>
        </div>
      </div>
    </div>
  );
};

export default Home;
