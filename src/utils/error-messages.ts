/**
 * Mapeo de errores de transacciones de Stellar a mensajes amigables en español
 */
export interface StellarErrorResponse {
    response?: {
        data?: {
            extras?: {
                result_codes?: {
                    transaction?: string;
                    operations?: string[];
                };
            };
            detail?: string;
            type?: string;
            title?: string;
        };
    };
    message?: string;
}

export const STELLAR_ERROR_MESSAGES: Record<string, string> = {
    // Errores de transacción
    tx_malformed: "La transacción está mal formada. Por favor intenta de nuevo.",
    tx_failed: "La transacción falló en la red. Revisa los detalles de la operación.",
    tx_too_early: "La transacción es demasiado temprana. Espera un momento e intenta de nuevo.",
    tx_too_late: "La transacción es demasiado tardía. Intenta de nuevo.",
    tx_duplicate: "Esta transacción ya fue procesada.",
    tx_insufficient_fee: "La tarifa de la transacción es insuficiente.",
    tx_internal_error: "Error interno en la red. Por favor intenta más tarde.",
    
    // Errores de operación comunes
    op_no_account: "La cuenta no existe en la red.",
    op_no_source_account: "La cuenta de origen no existe.",
    op_bad_auth: "Autorización fallida. Verifica que tengas permisos para esta operación.",
    op_bad_seq: "El número de secuencia es incorrecto.",
    op_not_supported: "Esta operación no está soportada.",
    op_too_many_subentries: "La cuenta tiene demasiadas subentradas.",
    op_exceeded_work_limit: "Se excedió el límite de trabajo de la operación.",
    op_bad_auth_extra: "Error de autorización adicional.",
    op_inflation_dest_not_found: "El destino de inflación no fue encontrado.",
    
    // Errores de contrato
    op_contract_execution_failed: "La ejecución del contrato falló. Verifica los parámetros.",
    op_soroban_resource_limit_exceeded: "Se excedió el límite de recursos de Soroban.",
    op_invalid_contract_auth: "Autorización de contrato inválida.",
};

/**
 * Obtiene un mensaje de error amigable basado en el código de error de Stellar
 */
export function getStellarErrorMessage(error: unknown): string {
    const stellarError = error as StellarErrorResponse;
    
    // Verificar si hay códigos de resultado de Stellar
    const resultCodes = stellarError?.response?.data?.extras?.result_codes;
    
    if (resultCodes) {
        // Priorizar errores de operación (más específicos)
        if (resultCodes.operations && resultCodes.operations.length > 0) {
            const opError = resultCodes.operations[0];
            if (STELLAR_ERROR_MESSAGES[opError]) {
                return STELLAR_ERROR_MESSAGES[opError];
            }
            // Si no hay mensaje específico, intentar usar el código directamente
            return `Error en la operación: ${opError}`;
        }
        
        // Si no hay error de operación, usar el error de transacción
        if (resultCodes.transaction) {
            const txError = resultCodes.transaction;
            if (STELLAR_ERROR_MESSAGES[txError]) {
                return STELLAR_ERROR_MESSAGES[txError];
            }
            return `Error en la transacción: ${txError}`;
        }
    }
    
    // Verificar si hay un mensaje de error directo
    if (stellarError?.response?.data?.detail) {
        return stellarError.response.data.detail;
    }
    
    if (stellarError?.response?.data?.title) {
        return stellarError.response.data.title;
    }
    
    // Verificar el mensaje del error
    if (stellarError?.message) {
        return stellarError.message;
    }
    
    // Error genérico
    return "Error desconocido al procesar la transacción. Por favor intenta de nuevo.";
}

/**
 * Verifica si un error es un error de Stellar
 */
export function isStellarError(error: unknown): error is StellarErrorResponse {
    return (
        typeof error === "object" &&
        error !== null &&
        "response" in error &&
        typeof (error as StellarErrorResponse).response === "object"
    );
}

