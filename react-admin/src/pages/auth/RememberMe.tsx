// src/components/RememberMe.tsx
import { FC } from "react";
import { useTranslation } from "react-i18next";



const RememberMe = ({email}:any) => {
    const [t, i18n] = useTranslation("global");
console.log("email::: ", email);
    const remember = (e:any)=>{
        if (e.target.checked) {
            localStorage.setItem("email", email);
        } else {
            localStorage.removeItem("email");
        }
    }

    return (
        <div className="flex items-center">
            <input
                id="rememberMe"
                type="checkbox"
                onChange={remember}
                className="h-4 w-4 text-primary-600 border-gray-300 rounded focus:ring-primary-500"
            />
            <label htmlFor="rememberMe" className="ml-2 block text-sm text-gray-900">
                {t('remember_me')}
            </label>
        </div>
    );
}

export default RememberMe;
