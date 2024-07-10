import React, { useEffect, useState } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import './home.css';
import DeviceButton from '../components/deviceButton';
import LockButton from '../components/lockButton';

const Home = () => {
  const location = useLocation();
  const navigate = useNavigate();
  const [devicesStatus, setDevicesStatus] = useState(location.state?.devicesStatus || {
    luz: false,
    tranca: false,
    alarme: false,
    cortinas: false,
    robo: false,
    cafeteira: false,
    ar_condicionado: false,
    aquecedor: false,
  });
  const [lockStatus, setLockStatus] = useState({
    luz: false,
    tranca: false,
    alarme: false,
    cortinas: false,
    robo: false,
    cafeteira: false,
    ar_condicionado: false,
    aquecedor: false,
  });
  const [horaAtual, setHoraAtual] = useState(location.state?.hora_atual || 0);
  const [tempAtual, setTempAtual] = useState(location.state?.temp_atual || 12);

  const deviceOrder = ['luz', 'tranca', 'alarme', 'cortinas', 'robo', 'cafeteira', 'ar_condicionado', 'aquecedor'];

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

  const toggleDevice = async (device) => {
    const updatedStatus = !devicesStatus[device];

    try {
      const response = await fetch('http://127.0.0.1:8080/api/update', {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ [device]: updatedStatus }),
      });

      const data = await response.json();
      setDevicesStatus(data);
    } catch (error) {
      console.error('Erro ao atualizar dados:', error);
    }
  };

  const toggleLock = async (device) => {
    const updatedLockStatus = !lockStatus[device];

    try {
      const response = await fetch('http://127.0.0.1:8080/api/lock_device', {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ [`lock_${device}`]: updatedLockStatus }),
      });

      const data = await response.json();
      setLockStatus(data);
    } catch (error) {
      console.error('Erro ao atualizar status de bloqueio:', error);
    }
  };

  return (
    <div className="body">
      <div className="container">
        <div className="container-header">
          <p>Hora Atual: {horaAtual}</p>
          <p>Temperatura Atual: {tempAtual.toFixed(2)}</p>
        </div>
        <div className="devices-grid">
          {deviceOrder.map((device) => (
            <div key={device} className="device-container">
              <DeviceButton
                name={device}
                isActive={devicesStatus[device]}
                isLocked={lockStatus[device]}
                toggleDevice={toggleDevice}
              />
              <LockButton
                name={device}
                isLocked={lockStatus[device]}
                toggleLock={toggleLock}
              />
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default Home;
