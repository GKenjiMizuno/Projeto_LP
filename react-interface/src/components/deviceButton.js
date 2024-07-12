// src/components/DeviceButton.js
import React from 'react';
import './deviceButton.css';


const DeviceButton = ({ name, isActive, isLocked, toggleDevice }) => {
  const handleClick = () => {
    if (!isLocked) {
      toggleDevice(name);
    }
  };

  return (
    <div className={`device-button ${isActive ? 'active' : 'inactive'} ${isLocked ? 'locked' : ''}`} onClick={handleClick}>
      {name}
    </div>
  );
};

export default DeviceButton;
