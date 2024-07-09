// src/components/DeviceButton.js
import React from 'react';
import './deviceButton.css';

const DeviceButton = ({ name, isActive, toggleDevice }) => {
  const handleClick = () => toggleDevice(name);

  return (
    <div className={`device-button ${isActive ? 'active' : 'inactive'}`} onClick={handleClick}>
      {name}
    </div>
  );
};

export default DeviceButton;
