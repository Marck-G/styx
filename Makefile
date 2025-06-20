BANNER_SHOWN := .banner
INSTALL_PATH=/etc/styx
 #Variables de color para la consola
# Colores de texto (foreground)
BLACK_COLOR   := \033[30m
RED_COLOR     := \033[31m
GREEN_COLOR   := \033[32m
YELLOW_COLOR  := \033[33m
BLUE_COLOR    := \033[34m
MAGENTA_COLOR := \033[35m
CYAN_COLOR    := \033[36m
WHITE_COLOR   := \033[37m

# Estilos de texto
BOLD_TEXT     := \033[1m
FAINT_TEXT    := \033[2m # Menos brillante
ITALIC_TEXT   := \033[3m # No soportado por todas las terminales
UNDERLINE_TEXT := \033[4m

# Resetear todos los atributos de formato
RESET_COLOR   := \033[0m

$(BANNER_SHOWN):
	@printf "$(GREEN_COLOR)"
	@printf "**************************************************************\n"
	@printf "**************************************************************\n"
	@printf "**             _____ _______   ____   __                    **\n"
	@printf "**            /  ___|_   _\ \ / /\ \ / /                    **\n"
	@printf "**            \ \`--.  | |  \ V /  \ V /                     **\n"
	@printf "**             \`--. \ | |   \ /   /   \                     **\n"
	@printf "**            /\__/ / | |   | |  / /^\ \                    **\n"
	@printf "**            \____/  \_/   \_/  \/   \/                    **\n"
	@printf "**                                                          **\n"
	@printf "**************************************************************\n"
	@printf "**************************************************************\n\n"
	@printf "$(RESET_COLOR)"
	@printf "$(YELLOW_COLOR)"
	@printf "█████████████████\t\t"
	@printf "$(RED_COLOR)"
	@printf "█████████████████\n"
	@printf "$(YELLOW_COLOR)"
	@printf "█████████████████\t\t"
	@printf "█████████████████\n"
	@printf "$(BLUE_COLOR)"
	@printf "█████████████████\t\t" 
	@printf "$(YELLOW_COLOR)"
	@printf "█████████████████\n"
	@printf "$(RED_COLOR)"
	@printf "█████████████████\t\t" 
	@printf "$(RED_COLOR)"
	@printf "█████████████████\n"
	@printf "$(RESET_COLOR)"
	@printf "       ECU      \t\t       ESP\n"
	@touch $@

build: $(BANNER_SHOWN) http rust _rest_banner
	@printf "\n\n"
	@echo "         ====================================="
	@echo "         *                                   *"
	@echo "         *          BUILD FINISH             *"
	@echo "         *                                   *"
	@echo "         ====================================="
	@printf "\n\n"
	@echo ""

	
clean: $(BANNER_SHOWN) clean-rs clean-web _rest_banner

_rest_banner:
	@rm -f $(BANNER_SHOWN)


rust:
	cargo build --release

clean-rs:
	cargo clean

clean-web:
	@rm -rf ./http
	@rm -rf ./webui/dist
	@echo "Clean Web files"


http:
	@cd ./webui && npm i && npm run build
	@cp -r ./webui ./http


install:
	@mkdir -p $(INSTALL_PATH)/lib
	@cp -r ./http $(INSTALL_PATH)/http
	@cp config.lua $(INSTALL_PATH)/config.lua
	@cp ./assets/styx.service /etc/systemd/system/styx.service
	@cp ./target/release/styx $(INSTALL_PATH)/styx
	@printf "\n\tDONE!\n"


.PHONY: buil clean clean-rs rust clean-web build install _rest_banner