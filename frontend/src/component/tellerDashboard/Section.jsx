import React from 'react';
import './Section.css';
function Section (props){

    return(
 <div className="widget">
     <div className='wleft'><b>{props.content}</b></div>
     <div className='wright'> </div>            
 </div>

    )
}
export default Section;