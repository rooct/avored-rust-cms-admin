import logo from "../../assets/logo_only.svg"
import { Link } from "react-router-dom"
import InputField from "../../components/InputField"
import { SubmitHandler, useForm } from "react-hook-form"
import { joiResolver } from "@hookform/resolvers/joi"
import { useLogin } from "./hooks/useLogin"
import { useLoginSchema } from "./schemas/login.schema"
import ErrorMessage from "../../components/ErrorMessage"
import { useTranslation } from "react-i18next"
import ILoginPost from "../../types/auth/ILoginPost"
import AvoRedButton from "../../components/AvoRedButton"
import { useEffect, useState } from "react"

function Login() {
    const [t, i18n] = useTranslation("global")
    const [rememberMe, setRememberMe] = useState<boolean>(false) // State to track "Remember Me"
    const {
        register,
        handleSubmit,
        formState: { errors },
        setValue
    } = useForm<ILoginPost>({
        resolver: joiResolver(useLoginSchema()),
    })
    const {
        mutate,
        isPending,
        error
    } = useLogin()

    // Load stored email if exists
    useEffect(() => {
        const storedEmail = localStorage.getItem("email")
        console.log("storedEmail::: ", storedEmail);
        if (storedEmail) {
            setValue("email", storedEmail)
        }
    }, [setValue])

    // Handle form submission
    const submitHandler: SubmitHandler<ILoginPost> = (data) => {
        console.log("data::: ", data);
        if (rememberMe) {
            console.log("rememberMe:3333:: ", data.email);
            localStorage.setItem("email", data.email) // Save email in localStorage if "Remember Me" is checked
        } else {
            localStorage.removeItem("email") // Remove email from localStorage if not checked
        }
        mutate(data)
    }

    return (
        <div className="min-h-screen bg-slate-100 flex flex-col justify-center py-12 sm:px-6 lg:px-8">
            <div className="flex justify-center">
                <img src={logo} className="w-20 h-20" alt={t("avored_rust_cms")} />
            </div>

            <div className="sm:mx-auto sm:w-full sm:max-w-md">
                <h2 className="mt-6 text-center text-3xl font-extrabold text-gray-900">
                    {t("sign_into_your_account")}
                </h2>
            </div>
            <div className="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
                <div className="bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10">
                    <form onSubmit={handleSubmit(submitHandler)} className="space-y-5">
                        <div>
                            <InputField
                                label={t("email")}
                                type="text"
                                name="email"
                                autoFocus={true}
                                register={register("email")}
                            />
                            <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="email" />
                        </div>
                        <div>
                            <InputField
                                label={t("password")}
                                type="password"
                                name="password"
                                register={register("password")}
                            />
                            <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="password" />
                        </div>

                        <div className="flex items-center justify-between">
                            <div className="flex items-center">
                                <input
                                    id="rememberMe"
                                    type="checkbox"
                                    checked={rememberMe}
                                    onChange={e => {
                                        setRememberMe(e.target.checked)
                                    }}
                                    className="h-4 w-4 text-primary-600 border-gray-300 rounded focus:ring-primary-500"
                                />
                                <label htmlFor="rememberMe" className="ml-2 block text-sm text-gray-900">
                                    {t('remember_me')}
                                </label>
                            </div>
                            <div className="text-sm">
                                <Link
                                    to={`/admin/forgot-password`}
                                    className="font-medium text-primary-600 hover:text-primary-500"
                                >
                                    {t("forgot_your_password")}
                                </Link>
                            </div>
                        </div>

                        <div>
                            <AvoRedButton
                                label={t("sign_in")}
                                isPending={isPending}
                                className="bg-primary-600 hover:bg-primary-500 focus:ring-primary-500"
                            />
                        </div>
                    </form>
                </div>
            </div>
        </div>
    );
}

export default Login;
