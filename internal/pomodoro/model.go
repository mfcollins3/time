// Copyright 2025 Michael F. Collins, III
//
// Time Source-Available Temporary License (v0.1)
//
// IMPORTANT: This is a source-available license and is not an open source
// license. The rights granted below are limited, temporary, and subject to
// change upon a future commercial release of the software.
//
// 1. Definitions
//
// - "Software" means the Time product, including its source code, build
//   scripts, configuration, and associated assets contained in this
//   repository.
// - "Derivative Works" means any work based upon the Software, including
//   modifications, adaptations, enhancements, or other works that are based on
//   or incorporate the Software, in whole or in part.
// - "Internal Use" means use by an individual or within a single legal entity
//   (including its employees and contractors acting on its behalf) for its own
//   business or personal operations, and not for providing the Software or any
//   substantial part of it to any third party as a hosted service, product, or
//   commercial offering.
// - "Integrations and Extensions" means separately distributed modules,
//   plugins, connectors, or tools that interact with the Software through
//   documented interfaces, data formats, or APIs, but which are not a
//   substantial copy of the Software itself.
// - "Commercial Release" means a future releaase of the Software made
//   available by the copyright holder under a different license, including a
//   commercial license.
//
// 2. License Grant (Temporary, Source-Available)
//
// Subject to the terms and conditions of this License, the copyright holder
// grants you a limited, non-exclusive, non-transferable, revocable license to:
//
// - Read and inspect the source code to learn how the Software is built and
//   understand the underlying technology (educational use);
// - Use, compile, and run the Software for your Internal Use;
// - Create Derivative Works of the Software for your Internal Use;
// - Develop Integrations and Extensions that interface with the Software;
// - Share your Derivative Works with other individuals or organizations solely
//   for their Internal Use, provided that (a) you and the recipient do not
//   charge any fees or other consideration for such Derivative Works or their
//   use, (b) you include this License and all applicable copyright notices
//   with any distribution, and (c) the recipient is bound by terms no less
//   protective of the Software and the copyright holder than these.
//
// This License is temporary and remains in effect only until the Commercial
// Release, as described below.
//
// 3. Restrictions
//
// Except as expressly permitted above, you may not:
//
// - Use the Software, Derivative Works, or Integration and Extensions for any
//   commercial purpose, including selling access, offering as-a-service
//   (SaaS), charging for hosting, support, or distribution, or otherwise
//   monetizing the Software or Derivative Works;
// - Distribute the Software or Derivative Works to the public for general use,
//   whether free or paid, other than sharing Derivative Works solely for a
//   recipient's Internal Use as allowed above;
// - Re-license, sublicense, or otherwise transfer rights in the Software,
//   except as expressly allowed herein;
// - Remove, alter, or obscure any copyright, license, or attribution notices;
// - Use the Software, trademarks, logos, or trade dress of the Time product or
//   the copyright holder in any way that suggests endorsement or affiliation
//   without prior written permission;
// - Use the Software to create a substantially similar competing product that
//   you distribute or offer to others.
//
// 4. Future Commercial License Requirement (Temporary Nature)
//
// This License is a temporary license intended to make the Software source
// available for learning, evaluation, and development of Integrations and
// Extensions prior to a commercial offering. Upon the Commercial Release:
//
// - Continued use of the Software or any Derivative Works derived from it
//   will require you (any any recipients with whom you have shared Derivative
//   Works) to obtain a commercial license within 30 days of the Commercial
//   Release; and
// - If you do not obtain a commercial license within that period, all rights
//   granted under this License automatically terminate, and you must cease use
//   and distribution of the Software and any Derivative Works.
//
// Noting in this License obligates the copyright holder to make a Commercial
// Release, but if/when a Commercial Release occurs, it will be under different
// terms, which may include paid licensing requirements.
//
// 5. Contributions
//
// If you submit, propose, or otherwise contribute bug fixes, features,
// enhancements, documentation, or other materials to the Software
// ("Contributions"):
//
// - You represent that you have the necessary rights to make the Contributions
//   and that your Contributions do not infringe third-party rights;
// - You agree your Contributions will be made available under the same License
//   as the Software at the time of contribution, without compensation to you;
// - You grant the copyright holder a perpetual, irrevocable, royalty-free,
//   worldwide, non-exclusive license to use, reproduce, modify, distribute,
//   sublicense, and create derivative works from your Contributions, and to
//   include them in the Software and in future versions and distributions;
// - You agree that making a Contribution does not grant you any ownership,
//   co-ownership, or any other rights in or to the Software, any subsequent
//   releases, or any licenses to continue using the Software beyond the rights
//   expressly granted by this License.
//
// 6. Ownership
//
// The Software is licensed, not sold. All rights, title, and interest in and
// to the Software (including all intellectual property rights) are and shall
// remain with the copyright holder and its licensors. Except for the limited
// rights expressly granted herin, no other rights are granted by implciation,
// estoppel, or otherwise.
//
// 7. Warranty Disclaimer
//
// THE SOFTWARE IS PROVIDED "AS IS" AND "AS AVAILABLE", WITHOUT WARRANTY OF ANY
// KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE, TITLE, AND
// NON-INFRINGEMENT. YOU ASSUME ALL RISKS ASSOCIATED WITH THE USE OF THE
// SOFTWARE.
//
// 8. Limitation of Liability
//
// TO THE MAXIMUM EXTENT PERMITTED BY APPLICABLE LAW, IN NO EVENT SHALL THE
// COPYRIGHT HOLDER OR ITS CONTRIBUTORS BE LIABLE FOR ANY INDIRECT, INCIDENTAL,
// SPECIAL, CONSEQUENTIAL, EXEMPLARY, OR PUNITIVE DAMAGES, OR ANY LOSS OF
// PROFITS, REVENUE, DATA, GOODWILL, OR BUSINESS INTERRUPTION, ARISING OUT OF
// OR RELATED TO THIS LICENSE OR THE USE OF OR INABILITY TO USE THE SOFTWARE,
// EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGES. THE TOTAL AGGREGATE
// LIABILITY OF THE COPYRIGHT HOLDER FOR ANY CLAIMS ARISING OUR OF OR RELATING
// TO THIS LICENSE OR THE SOFTWARE SHALL NOT EXCEED ONE HUNDRED U.S. DOLLARS
// (UD$100).
//
// 9. Termination
//
// This License automatically terminates if you breach any of its terms or
// upon the conditions described in Section 4. Upon termination, you must
// immediately cease all use and distribution of the Software and Derivative
// Works and destroy all copies in your possession or control. Sections that by
// their nature should survive termination shall survive (including, without
// limitation, Sections 5-8 and 10-12).
//
// 10. Third-Party Components
//
// The Software may include or depend on third-party components that are
// provided under their own licenses. Such licenses govern those components,
// and nothing in this License purports to limit your rights under, or impose
// obligations beyond, the applicable third-party licenses.
//
// 11. Export and Compliance
//
// You are responsible for complying with all applicable laws and regulations,
// including export control laws, in connection with your use of the Software.
//
// 12. Entire Agreement; Changes
//
// This License constitutes the entire agreement between you and the copyright
// holder regarding the Software. The copyright holder may update this License
// for future releases. Your continued use of future releases is subject to the
// then-current license accompanying those releases.
//
// For inquiries about commercial licensing, please contact the copyright
// holder.

package pomodoro

import (
	"context"
	"fmt"
	"time"

	"github.com/charmbracelet/bubbles/progress"
	tea "github.com/charmbracelet/bubbletea"
	"michaelfcollins3.dev/projects/time/internal/dbid"
)

type model struct {
	ctx              context.Context
	startTime        time.Time
	minutesRemaining int
	secondsRemaining int
	progress         progress.Model
	err              error
	pomodoroID       dbid.ID
}

func newModel(ctx context.Context, startTime time.Time) model {
	return model{
		ctx:              ctx,
		startTime:        startTime,
		minutesRemaining: 25,
		secondsRemaining: 0,
		progress: progress.New(
			progress.WithSolidFill("#00ff00"),
			progress.WithWidth(50),
			progress.WithoutPercentage(),
		),
	}
}

func (m model) Init() tea.Cmd {
	return tea.Batch(
		createPomodoro(m.ctx, m.startTime),
		tick(),
	)
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tickMsg:
		m = m.updateTime(msg)
		percent := 1 - (float64(m.minutesRemaining*60+m.secondsRemaining) /
			float64(pomodoroDuration.Seconds()))
		if percent >= 0.8 {
			m.progress.FullColor = "#ffff00"
		}

		if percent >= 0.95 {
			m.progress.FullColor = "#ff0000"
		}

		return m, tea.Batch(
			m.progress.SetPercent(percent),
			tick(),
		)

	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c", "q":
			return m, tea.Quit
		}

	case progress.FrameMsg:
		progressModel, cmd := m.progress.Update(msg)
		m.progress = progressModel.(progress.Model)
		return m, cmd

	case errorMsg:
		m.err = msg.err
		return m, tea.Quit

	case pomodoroCreatedMsg:
		m.pomodoroID = msg.ID
		return m, nil
	}

	return m, nil
}

func (m model) View() string {
	return fmt.Sprintf(
		"%s %d:%02d",
		m.progress.View(),
		m.minutesRemaining,
		m.secondsRemaining,
	)
}

func (m model) updateTime(msg tickMsg) model {
	timeRemaining := pomodoroDuration - msg.time.Sub(m.startTime)
	expired := timeRemaining <= 0
	if expired {
		m.minutesRemaining = 0
		m.secondsRemaining = 0
	} else {
		m.minutesRemaining = int(timeRemaining.Minutes())
		m.secondsRemaining = int(timeRemaining.Seconds()) % 60
	}

	return m
}
