import { useEffect } from "react";
import { useNavigate } from "react-router-dom";

function Logout() {
    const redirect = useNavigate();
    useEffect(() => {
        let email = localStorage.getItem("email");
        console.log("email::22: ", email);
        localStorage.clear();
        if(email){
            localStorage.setItem("email", email);
        }
        redirect("/admin/login");
    }, [redirect]); // Added "redirect" to the dependency array

    return (
        <></>
    )
}

export default Logout;
