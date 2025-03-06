# ==============================================================================
# SPDX-License-Identifier: CC0-1.0
# SPDX-FileType: SOURCE
# SPDX-FileCopyrightText: (c) 2022, OpenGateware authors and contributors
# ==============================================================================
# DE10-Nano GPIO 0
# ==============================================================================
post_message "Framework: Loading GPIO 0 pinout"

# ==============================================================================
# SDRAM
# ==============================================================================
set_location_assignment PIN_AH8 -to SDRAM_A[0]
set_location_assignment PIN_AG9 -to SDRAM_A[1]
set_location_assignment PIN_AH7 -to SDRAM_A[2]
set_location_assignment PIN_AG8 -to SDRAM_A[3]
set_location_assignment PIN_AH13 -to SDRAM_A[4]
set_location_assignment PIN_AF15 -to SDRAM_A[5]
set_location_assignment PIN_AH14 -to SDRAM_A[6]
set_location_assignment PIN_AF17 -to SDRAM_A[7]
set_location_assignment PIN_AG16 -to SDRAM_A[8]
set_location_assignment PIN_Y18 -to SDRAM_A[9]
set_location_assignment PIN_AG10 -to SDRAM_A[10]
set_location_assignment PIN_Y17 -to SDRAM_A[11]
set_location_assignment PIN_AG18 -to SDRAM_A[12]
set_location_assignment PIN_AH11 -to SDRAM_BA[0]
set_location_assignment PIN_AH9 -to SDRAM_BA[1]
set_location_assignment PIN_AF28 -to SDRAM_DQ[0]
set_location_assignment PIN_AF27 -to SDRAM_DQ[1]
set_location_assignment PIN_AG28 -to SDRAM_DQ[2]
set_location_assignment PIN_AH27 -to SDRAM_DQ[3]
set_location_assignment PIN_AG26 -to SDRAM_DQ[4]
set_location_assignment PIN_AH26 -to SDRAM_DQ[5]
set_location_assignment PIN_AG25 -to SDRAM_DQ[6]
set_location_assignment PIN_AG24 -to SDRAM_DQ[7]
set_location_assignment PIN_AG20 -to SDRAM_DQ[8]
set_location_assignment PIN_AG19 -to SDRAM_DQ[9]
set_location_assignment PIN_AG21 -to SDRAM_DQ[10]
set_location_assignment PIN_AH21 -to SDRAM_DQ[11]
set_location_assignment PIN_AH23 -to SDRAM_DQ[12]
set_location_assignment PIN_AH22 -to SDRAM_DQ[13]
set_location_assignment PIN_AH24 -to SDRAM_DQ[14]
set_location_assignment PIN_AG23 -to SDRAM_DQ[15]
set_location_assignment PIN_AH19 -to SDRAM_CLK
set_location_assignment PIN_AG14 -to SDRAM_nWE
set_location_assignment PIN_AH12 -to SDRAM_nCAS
set_location_assignment PIN_AG11 -to SDRAM_nCS
set_location_assignment PIN_AG13 -to SDRAM_nRAS

set_instance_assignment -name VIRTUAL_PIN ON -to SDRAM_DQML
set_instance_assignment -name VIRTUAL_PIN ON -to SDRAM_DQMH
set_instance_assignment -name VIRTUAL_PIN ON -to SDRAM_CKE

set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_*
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_*
set_instance_assignment -name FAST_OUTPUT_REGISTER ON -to SDRAM_*
set_instance_assignment -name FAST_OUTPUT_ENABLE_REGISTER ON -to SDRAM_DQ[*]
set_instance_assignment -name FAST_INPUT_REGISTER ON -to SDRAM_DQ[*]
set_instance_assignment -name ALLOW_SYNCH_CTRL_USAGE OFF -to *|SDRAM_*

set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_A[0]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_A[1]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_A[2]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_A[3]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_A[4]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_A[5]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_A[6]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_A[7]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_A[8]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_A[9]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_A[10]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_A[11]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_A[12]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_BA[0]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_BA[1]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_DQ[0]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_DQ[1]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_DQ[2]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_DQ[3]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_DQ[4]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_DQ[5]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_DQ[6]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_DQ[7]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_DQ[8]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_DQ[9]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_DQ[10]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_DQ[11]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_DQ[12]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_DQ[13]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_DQ[14]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_DQ[15]
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_CLK
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_nWE
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_nCAS
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_nCS
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_nRAS
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_DQML
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_DQMH
set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SDRAM_CKE

set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_A[0]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_A[1]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_A[2]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_A[3]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_A[4]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_A[5]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_A[6]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_A[7]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_A[8]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_A[9]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_A[10]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_A[11]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_A[12]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_BA[0]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_BA[1]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_DQ[0]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_DQ[1]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_DQ[2]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_DQ[3]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_DQ[4]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_DQ[5]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_DQ[6]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_DQ[7]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_DQ[8]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_DQ[9]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_DQ[10]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_DQ[11]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_DQ[12]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_DQ[13]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_DQ[14]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_DQ[15]
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_CLK
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_nWE
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_nCAS
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_nCS
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_nRAS
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_DQML
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_DQMH
set_instance_assignment -name CURRENT_STRENGTH_NEW "MAXIMUM CURRENT" -to SDRAM_CKE

set_instance_assignment -name FAST_INPUT_REGISTER ON -to SDRAM_DQ[*]
set_instance_assignment -name FAST_OUTPUT_REGISTER ON -to SDRAM_*
set_instance_assignment -name FAST_OUTPUT_ENABLE_REGISTER ON -to SDRAM_DQ[*]
set_instance_assignment -name ALLOW_SYNCH_CTRL_USAGE OFF -to *|SDRAM_*
