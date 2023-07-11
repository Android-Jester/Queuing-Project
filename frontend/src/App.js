import './component/Login.css';
import './component/Guestlogin.css';
import './component/CustomerPage.css'
import './App.css';
import './component/QueuePage.css';
import './component/TellerLogin.css';
import Login from './component/Login';
import {BrowserRouter,Routes,Route} from 'react-router-dom'
import Guestlogin from './component/Guestlogin';
import CustomerPage from './component/CustomerPage';
import QueuePage from './component/QueuePage';
import TellerLogin from './component/TellerLogin';
import React, {useState} from 'react';
import Dashboard from "./component/tellerDashboard/Dashboard";
import 'react-circular-progressbar/dist/styles.css';
import { UserContext } from './component/UserContext';



function App() {
  const [reValue,setReValue]= useState('helloo');
  return (
  <BrowserRouter>
   <UserContext.Provider value={{reValue,setReValue}}>
    <Routes>
   
     <Route path='/'>
      <Route index element={<Login/>} />
      <Route path="Guestlogin" element={<Guestlogin/>}/>
      <Route path="CustomerPage" element={<CustomerPage/>}/>
      <Route path="QueuePage" element={<QueuePage/>}/>
      </Route>
      <Route path="Tella">
      <Route index element={<TellerLogin/>}/>
      <Route path="Dashboard" element={<Dashboard/>}/> 
      </Route>
      
    </Routes>
    </UserContext.Provider>
   </BrowserRouter>
 
    
  )
}

export default App;
