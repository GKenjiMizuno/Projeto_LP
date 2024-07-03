import './App.css';
import Login from './screens/login';
import Home from './screens/home';
import DataFetcher from './DataFetcher';
import { BrowserRouter as Router, Route, Switch, Routes } from 'react-router-dom';

function App() {
  return (
    <Router>
      <div>
        <Routes>
          <Route exact path='/' element={<Login />} />
          <Route path = '/home' element={<Home />} />
        </Routes>
      </div>
    </Router>
  );
}

export default App;
