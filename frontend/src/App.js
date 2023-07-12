import React, { useState } from 'react';
import 'react-circular-progressbar/dist/styles.css';
import { BrowserRouter, Route, Routes } from 'react-router-dom';
import './App.css';
import CustomerPage from './component/CustomerPage';
import './component/CustomerPage.css';
import Guestlogin from './component/Guestlogin';
import './component/Guestlogin.css';
import Login from './component/Login';
import './component/Login.css';
import QueuePage from './component/QueuePage';
import './component/QueuePage.css';
import TellerLogin from './component/TellerLogin';
import './component/TellerLogin.css';
import { UserContext } from './component/UserContext';
import Dashboard from "./component/tellerDashboard/Dashboard";



function App() {
  let [reValue, setReValue] = useState();
  // const value_ref = useRef('');
  return (
    <BrowserRouter>
      <UserContext.Provider value={{ reValue, setReValue }}>
        {/* <UserContext.Provider value={{ value_ref }}> */}
        <Routes>

          <Route path='/'>
            <Route index element={<Login />} />
            <Route path="Guestlogin" element={<Guestlogin />} />
            <Route path="CustomerPage" element={<CustomerPage />} />
            <Route path="QueuePage" element={<QueuePage />} />
          </Route>
          <Route path="Tella">
            <Route index element={<TellerLogin />} />
            <Route path="Dashboard" element={<Dashboard />} />
          </Route>

        </Routes>
      </UserContext.Provider>
    </BrowserRouter>


  )
}

export default App;
