import { NavLink, Outlet } from "react-router-dom";
import { Menu } from "@headlessui/react";
import { useTranslation } from "react-i18next";
import { ChevronDownIcon, FilmIcon, RocketLaunchIcon, CpuChipIcon, DeviceTabletIcon } from "@heroicons/react/24/solid";
import { useLoggedInUser } from "../../hooks/useLoggedInUser";

function AppSidebar() {
    const [t] = useTranslation("global");
    const logged_in_user = useLoggedInUser();

    // Define sidebar menu items in an array
    const menuItems = [
        {
            to: "/admin",
            icon: <CpuChipIcon className="h-4 w-4" />,
            label: t("sidebar.dashboard")
        },
        {
            to: "/admin/page",
            icon: <RocketLaunchIcon className="h-4 w-4" />,
            label: t("sidebar.page")
        },
        {
            to: "/admin/component",
            icon: <CpuChipIcon className="h-4 w-4" />,
            label: t("components")
        },
        {
            to: "/admin/asset",
            icon: <FilmIcon className="h-4 w-4" />,
            label: t("asset_manager")
        },
        {
            to: "/admin/model",
            icon: <DeviceTabletIcon className="h-4 w-4" />,
            label: t("model")
        }
    ];

    // Define management items for super admins
    const managementItems = [
        {
            to: "/admin/admin-user",
            label: t("sidebar.admin_user")
        },
        {
            to: "/admin/role",
            label: t("sidebar.role")
        },
        {
            to: "/admin/setting",
            label: t("sidebar.setting")
        }
    ];

    return (
        <div className="flex">
            <div className="w-64 max-h-screen top-0 pt-16 h-screen bg-gray-800 text-blue-100 fixed inset-y-0 left-0 transform transition duration-200 ease-in-out">
                <nav className="px-4 pt-4 scroller max-h-[calc(100vh-64px)]">
                    <ul className="flex flex-col space-y-2">
                        {/* Render Dashboard Menu Item */}
                        <li className="text-sm text-gray-500">
                            <NavLink
                                to={menuItems[0].to}
                                className={({ isActive }) =>
                                    isActive
                                        ? "flex items-center w-full py-1 px-2 rounded relative text-white bg-gray-700"
                                        : "flex items-center w-full py-1 px-2 rounded relative hover:text-white hover:bg-gray-700"
                                }
                            >
                                <div className="pr-2">{menuItems[0].icon}</div>
                                <div className="text-gray-200">{menuItems[0].label}</div>
                            </NavLink>
                        </li>

                        {/* Section Title */}
                        <div className="section border-b pt-4 mb-4 text-xs text-gray-600 border-gray-700 pb-1 pl-3">
                            {t("sidebar.content_manager")}
                        </div>

                        {/* Render other menu items */}
                        {menuItems.slice(1).map((item, index) => (
                            <li key={index} className="text-sm text-gray-200 ">
                                <NavLink
                                    to={item.to}
                                    className={({ isActive }) =>
                                        isActive
                                            ? "flex items-center w-full py-1 px-2 mt-3 rounded relative font-bold text-white bg-gray-700"
                                            : "flex items-center w-full py-1 px-2 mt-3 rounded relative hover:text-white hover:bg-gray-700"
                                    }
                                >
                                    <div className="pr-2">{item.icon}</div>
                                    <div >{item.label}</div>
                                </NavLink>
                            </li>
                        ))}

                        {/* Conditionally render management section for super admins */}
                        {logged_in_user.is_super_admin && (
                            <>
                                <div className="section border-b pt-4 mb-4 text-xs text-gray-600 border-gray-700 pb-1 pl-3">
                                    {t("sidebar.management")}
                                </div>
                                <Menu as="li" className="text-sm text-gray-200">
                                    <Menu.Button className="flex items-center w-full py-1 px-2 mt-3 rounded relative hover:text-white hover:bg-gray-700">
                                        <div className="pr-2">
                                            <RocketLaunchIcon className="h-4 w-4" />
                                        </div>
                                        <div>{t("sidebar.team")}</div>
                                        <div className="absolute right-1.5 transition-transform duration-300">
                                            <ChevronDownIcon className="h-4 w-4" />
                                        </div>
                                    </Menu.Button>
                                    <Menu.Items className="flex flex-col mt-2 pl-2 ml-3 border-l border-gray-700 space-y-1">
                                        {managementItems.map((item, index) => (
                                            <Menu.Item as="li" key={index}>
                                                {({ active }) => (
                                                    <NavLink
                                                        to={item.to}
                                                        className={({ isActive }) =>
                                                            isActive
                                                                ? "flex items-center w-full py-1 px-2 rounded relative font-bold text-white bg-gray-700"
                                                                : "flex items-center w-full py-1 px-2 rounded relative hover:text-white hover:bg-gray-700"
                                                        }
                                                    >
                                                        <div>{item.label}</div>
                                                    </NavLink>
                                                )}
                                            </Menu.Item>
                                        ))}
                                    </Menu.Items>
                                </Menu>
                            </>
                        )}
                    </ul>
                </nav>
            </div>
            <Outlet />
        </div>
    );
}

export default AppSidebar;
