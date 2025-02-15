import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useAxios } from "../../../hooks/useAxios";
import IAssetSave from "../../../types/asset/IAssetSave";
import {CreateFolderType} from "../../../types/asset/CreateFolderType";
import {DeleteFolderType} from "../../../types/asset/DeleteFolderType";

export const useDeleteAsset = () => {
    const client = useAxios();
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: async (data: DeleteFolderType) => {
            console.log("datat-",data)
            return await client.delete(`/delete-asset/${data.asset_id}`);
        },
        onSuccess: (res) => {
            queryClient.invalidateQueries({ queryKey: ["asset-table"] });
        },
    });
};
